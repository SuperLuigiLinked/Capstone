/*
 *  Crate: RGE
 * Module: Vulkan - Render
 */

//! Internal functionality to Render state provided by the RGE Game Engine to a Window, using Vulkan.

// ================================================================================================================================ //

#[allow(unused_imports)]
use super::*;

// ================================================================================================================================ //

impl VulkanRenderable {
    /// Attempts to render the provided RGE State.
    ///
    /// If either of the Window dimensions (width or height) is 0, this function will return without rendering.\
    /// In some cases (such as the Window being resized), rendering may spuriously fail.\
    /// In such a case, the Swapchain will be re-created, and it will attempt to render again from the start.
    ///
    /// # Panics
    /// Panics if rendering fails several times in a row.
    pub fn render(&mut self, settings: &RenderSettings) {
        let mut fails = 0;

        loop {
            let (valid, changed_size) = self.update_size();
            let changed_vsync = self.frames.swapchain.vsync != settings.vsync;
            let changed = changed_size || changed_vsync;

            if !valid {
                //eprintln!("---- INVALID WINDOW ----");
                return;
            }

            if changed {
                self.update_surface_info();
                self.update_swapchain(settings.vsync);
            }

            if self.render_frame(settings) {
                // let _res = unsafe {
                //     let alpha = (settings.backcolor.3 * (u8::MAX as glsl::float)) as u8;
                //     sys::SetLayeredWindowAttributes(self.surface.window, 0, alpha, sys::LWA_ALPHA)
                // };
                break;
            }

            if !changed {
                self.update_surface_info();
                self.update_swapchain(settings.vsync);
            }

            {
                fails += 1;
                eprintln!("RENDER FAILED! [{fails}]");

                if fails == 5 {
                    panic!("Unable to render to Window! [Tried {fails} times]");
                }
            }
        }

        self.frame_idx = self.next_frame_idx();
    }

    // ================================================================================================================================ //

    /// Returns the Total Number of Frames that can be rendered simultaneously.
    fn number_frames(&self) -> usize {
        self.frames.list.len()
    }

    /// Returns the Index of the Next Frame to be acquired.
    fn next_frame_idx(&self) -> usize {
        (self.frame_idx + 1) % self.number_frames()
    }

    /// Returns the Index of the Previous Frame that was acquired.
    fn prev_frame_idx(&self) -> usize {
        if self.frame_idx == 0 {
            self.number_frames() - 1
        } else {
            self.frame_idx - 1
        }
    }

    // ================================================================================================================================ //

    /// Queries the Size of the Render Surface's Window and updates internal state as needed, returning boolean flags to indicate the changes.\
    /// The 0th flag represents whether the Window size is valid (non-zero in both dimensions).
    /// The 1st flag represents whether the Window size has changed.
    fn update_size(&mut self) -> (bool, bool) {
        let real_size = self.surface.real_size();

        let valid = (real_size.width != 0) && (real_size.height != 0);
        let changed = real_size != self.surface.size;

        self.surface.size = real_size;

        (valid, changed)
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Re-acquires Device/Surface Information.
    fn update_surface_info(&mut self) {
        let physical = unsafe { self.device().selection.physical() };
        self.ds_info = DeviceSurfaceInfo::new(physical, &self.surface);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Remakes the Swapchain from scratch.
    fn update_swapchain(&mut self, vsync: bool) {
        self.frame_idx = 0;

        self.frames.update(
            unsafe { self.device.as_ref() },
            &self.surface,
            &self.ds_info,
            &self.shaders.render_pass,
            vsync,
        );
    }

    // ================================================================================================================================ //

    /// Attempts to render a Frame.\
    /// Returns `true` if succeeded, and `false` if the swapchain needs updated.
    fn render_frame(&self, settings: &RenderSettings) -> bool {
        let prev_frame = self.frames.list.get(self.prev_frame_idx()).unwrap();
        let this_frame = self.frames.list.get(self.frame_idx).unwrap();

        let image_acquired = this_frame.sync.image.handle;
        let render_finished = this_frame.sync.render.handle;
        let this_frame_available = this_frame.sync.frame.handle;
        let prev_frame_available = prev_frame.sync.frame.handle;

        // -------------------------------- //

        // <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html>

        // -------------------------------- //

        let Some(image_idx) = self.acquire_image(image_acquired, &[this_frame_available, prev_frame_available], &[this_frame_available])
        else {
            return false;
        };
        assert_eq!(self.frame_idx, image_idx as usize);

        self.record_commands(this_frame, settings);

        if !self.submit_commands(
            this_frame,
            &[image_acquired],
            &[render_finished],
            this_frame_available,
        ) {
            return false;
        }

        if !self.present_image(image_idx, &[render_finished]) {
            return false;
        }

        true
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Attempts to acquires a Frame from the Swapchain.
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html>
    fn acquire_image(
        &self,
        signal_semaphore: vk::Semaphore,
        wait_fences: &[vk::Fence],
        reset_fences: &[vk::Fence],
    ) -> Option<u32> {
        let device = unsafe { self.device() };
        let ash_device = device.ash_device();
        let frames = &self.frames;
        let swapchain = &frames.swapchain;
        let swapchain_ext = unsafe { swapchain.ext() };

        // -------------------------------- //

        let res = unsafe { ash_device.wait_for_fences(wait_fences, true, u64::MAX) };
        res.unwrap();

        let res = unsafe {
            swapchain_ext.acquire_next_image(
                swapchain.handle,
                u64::MAX,
                signal_semaphore,
                vk::Fence::null(),
            )
        };
        let (image_idx, _suboptimal) = match res {
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return None,
            _res => _res.unwrap(),
        };

        let res = unsafe { ash_device.reset_fences(reset_fences) };
        res.unwrap();

        // -------------------------------- //

        Some(image_idx)
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Copies all the Byte Slices into a Memory-Mapped region.
    fn map_bytes<const N: usize>(
        memory: &DeviceMemory,
        map_offset: vk::DeviceSize,
        map_len: vk::DeviceSize,
        byte_slices: &[&[u8]; N],
    ) -> ([vk::DeviceSize; N], vk::DeviceSize) {
        let mut mem = unsafe { memory.map(map_offset, map_len) };

        let mut end = 0;
        let offsets = byte_slices.map(|bytes| {
            let offset = end;
            end = mem.copy_to(offset, bytes);
            offset
        });

        (offsets, end)
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Records the actual commands to render the Frame into a Command Buffer.
    fn record_commands(&self, frame: &Frame, settings: &RenderSettings) {
        let device = unsafe { self.device() };
        let ash_device = device.ash_device();

        let ds_info = &self.ds_info;
        let surface = &self.surface;
        let render_pass = &self.shaders.render_pass;

        // -------------------------------- //

        unsafe {
            ash_device
                .reset_command_buffer(frame.commands, vk::CommandBufferResetFlags::empty())
                .unwrap();
        }

        // -------------------------------- //

        let image_rect = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: ds_info.ideal_resolution(surface),
        };

        let (viewport, scissor) = ds_info.ideal_viewport_scissor(surface);

        let clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                // R G B A : [0.0 - 1.0]
                //float32: [0.0, 0.0, 0.5, 1.0],
                float32: [
                    settings.backcolor.0,
                    settings.backcolor.1,
                    settings.backcolor.2,
                    settings.backcolor.3,
                ],
            },
        };

        // ---------------------------------------------------------------- //
        // ======================= RENDERING  ORDER ======================= //
        // * UV Triangle Fans
        // *    Triangle Fans
        // * UV Triangle Strips
        // *    Triangle Strips
        // * UV Triangles
        // *    Triangles
        // * UV Line Strips
        // *    Line Strips
        // * UV Lines
        // *    Lines
        // * UV Points
        // *    Points
        // ---------------------------------------------------------------- //

        /// The total number of Graphics Pipelines.
        const PIPELINE_COUNT: usize = 12;

        let uv_vertices = [
            settings.uv_tri_fan.as_slice(),    // UV Triangle Fans
            settings.uv_tri_strip.as_slice(),  // UV Triangle Strips
            settings.uv_tri_list.as_slice(),   // UV Triangles
            settings.uv_line_strip.as_slice(), // UV Line Strips
            settings.uv_line_list.as_slice(),  // UV Lines
            settings.uv_point_list.as_slice(), // UV Points
        ];

        let rgba_vertices = [
            settings.tri_fan.as_slice(),    // Triangle Fans
            settings.tri_strip.as_slice(),  // Triangle Strips
            settings.tri_list.as_slice(),   // Triangles
            settings.line_strip.as_slice(), // Line Strips
            settings.line_list.as_slice(),  // Lines
            settings.point_list.as_slice(), // Points
        ];

        let indices = [
            settings.uv_tri_fan_idx.as_slice(),    // UV Triangle Fans
            settings.tri_fan_idx.as_slice(),       //    Triangle Fans
            settings.uv_tri_strip_idx.as_slice(),  // UV Triangle Strips
            settings.tri_strip_idx.as_slice(),     //    Triangle Strips
            &[],                                   // UV Triangles
            &[],                                   //    Triangles
            settings.uv_line_strip_idx.as_slice(), // UV Line Strips
            settings.line_strip_idx.as_slice(),    //    Line Strips
            &[],                                   // UV Lines
            &[],                                   //    Lines
            &[],                                   // UV Points
            &[],                                   //    Points
        ];

        let descriptor_sets = [
            Some(self.shaders.uv.descriptor_set), // UV Triangle Fans
            None,                                 //    Triangle Fans
            Some(self.shaders.uv.descriptor_set), // UV Triangle Strips
            None,                                 //    Triangle Strips
            Some(self.shaders.uv.descriptor_set), // UV Triangles
            None,                                 //    Triangles
            Some(self.shaders.uv.descriptor_set), // UV Line Strips
            None,                                 //    Line Strips
            Some(self.shaders.uv.descriptor_set), // UV Lines
            None,                                 //    Lines
            Some(self.shaders.uv.descriptor_set), // UV Points
            None,                                 //    Points
        ];

        let pipeline_handles = [
            self.shaders.uv.graphics.handles[5],   // UV Triangle Fans
            self.shaders.rgba.graphics.handles[5], //    Triangle Fans
            self.shaders.uv.graphics.handles[4],   // UV Triangle Strips
            self.shaders.rgba.graphics.handles[4], //    Triangle Strips
            self.shaders.uv.graphics.handles[2],   // UV Triangles
            self.shaders.rgba.graphics.handles[2], //    Triangles
            self.shaders.uv.graphics.handles[3],   // UV Line Strips
            self.shaders.rgba.graphics.handles[3], //    Line Strips
            self.shaders.uv.graphics.handles[1],   // UV Lines
            self.shaders.rgba.graphics.handles[1], //    Lines
            self.shaders.uv.graphics.handles[0],   // UV Points
            self.shaders.rgba.graphics.handles[0], //    Points
        ];

        let pipeline_layouts = [
            self.shaders.uv.pipeline_layout.handle,   // UV Triangle Fans
            self.shaders.rgba.pipeline_layout.handle, //    Triangle Fans
            self.shaders.uv.pipeline_layout.handle,   // UV Triangle Strips
            self.shaders.rgba.pipeline_layout.handle, //    Triangle Strips
            self.shaders.uv.pipeline_layout.handle,   // UV Triangles
            self.shaders.rgba.pipeline_layout.handle, //    Triangles
            self.shaders.uv.pipeline_layout.handle,   // UV Line Strips
            self.shaders.rgba.pipeline_layout.handle, //    Line Strips
            self.shaders.uv.pipeline_layout.handle,   // UV Lines
            self.shaders.rgba.pipeline_layout.handle, //    Lines
            self.shaders.uv.pipeline_layout.handle,   // UV Points
            self.shaders.rgba.pipeline_layout.handle, //    Points
        ];

        // ---------------------------------------------------------------- //

        let uv_vertex_counts =
            uv_vertices.map(|slice| vk::DeviceSize::try_from(slice.len()).unwrap());
        let rgba_vertex_counts =
            rgba_vertices.map(|slice| vk::DeviceSize::try_from(slice.len()).unwrap());
        let vertex_counts: [vk::DeviceSize; PIPELINE_COUNT] =
            utils::array_intersperse(uv_vertex_counts, rgba_vertex_counts);

        let uv_vertex_bytes = uv_vertices.map(utils::bytes_ref);
        let rgba_vertex_bytes = rgba_vertices.map(utils::bytes_ref);
        let vertex_bytes: [&[u8]; PIPELINE_COUNT] =
            utils::array_intersperse(uv_vertex_bytes, rgba_vertex_bytes);

        let vertex_bytes_len = vertex_bytes
            .map(|bytes| vk::DeviceSize::try_from(bytes.len()).unwrap())
            .iter()
            .sum();

        let index_counts = indices.map(|slice| vk::DeviceSize::try_from(slice.len()).unwrap());
        let index_bytes = indices.map(utils::bytes_ref);
        let index_bytes_len = index_bytes
            .iter()
            .map(|bytes| vk::DeviceSize::try_from(bytes.len()).unwrap())
            .sum();

        let (vertex_offsets, _vertex_end) = Self::map_bytes(
            &self.buffers.memory,
            self.buffers.vertex.offs,
            vertex_bytes_len,
            &vertex_bytes,
        );

        let (index_offsets, _index_end) = Self::map_bytes(
            &self.buffers.memory,
            self.buffers.index.offs,
            index_bytes_len,
            &index_bytes,
        );

        // ---------------------------------------------------------------- //

        /// Internal Struct for representing a Sub-Buffer.
        struct SubBuffer {
            /// Byte-Offset of the sub-buffer.
            offset: vk::DeviceSize,

            /// Byte-Count of the sub-buffer.
            count: vk::DeviceSize,
        }

        /// Internal Struct for wrapping a Pipeline and its associated Vertex and (Optional) Index Buffers.
        struct PipelineInfo {
            /// Pipeline Handle.
            handle: vk::Pipeline,

            /// Pipeline Layout.
            layout: vk::PipelineLayout,

            /// Descriptor Set.
            descriptor_set: Option<vk::DescriptorSet>,

            /// Vertex Buffer.
            vertex: SubBuffer,

            /// Index Buffer.
            index: Option<SubBuffer>,
        }

        // ---------------------------------------------------------------- //

        let pipelines = {
            assert_eq!(PIPELINE_COUNT, vertex_counts.len());
            assert_eq!(PIPELINE_COUNT, vertex_offsets.len());
            assert_eq!(PIPELINE_COUNT, index_counts.len());
            assert_eq!(PIPELINE_COUNT, index_offsets.len());
            assert_eq!(PIPELINE_COUNT, descriptor_sets.len());
            assert_eq!(PIPELINE_COUNT, pipeline_handles.len());
            assert_eq!(PIPELINE_COUNT, pipeline_layouts.len());

            let handles = pipeline_handles.iter();
            let layouts = pipeline_layouts.iter();
            let vtx_offsets = vertex_offsets.iter();
            let vtx_counts = vertex_counts.iter();
            let idx_offsets = index_offsets.iter();
            let idx_counts = index_counts.iter();
            let desc_sets = descriptor_sets.iter();

            let infos =
                handles.zip(layouts.zip(
                    vtx_offsets.zip(vtx_counts.zip(idx_offsets.zip(idx_counts.zip(desc_sets)))),
                ));

            infos.map(|info| {
                let handle = *info.0;
                let layout = *info.1 .0;
                let vtx_offset = *info.1 .1 .0;
                let vtx_count = *info.1 .1 .1 .0;
                let idx_offset = *info.1 .1 .1 .1 .0;
                let idx_count = *info.1 .1 .1 .1 .1 .0;
                let descriptor_set = *info.1 .1 .1 .1 .1 .1;

                let vertex = SubBuffer {
                    offset: vtx_offset,
                    count: vtx_count,
                };

                let index = if idx_count == 0 {
                    None
                } else {
                    Some(SubBuffer {
                        offset: idx_offset,
                        count: idx_count,
                    })
                };

                PipelineInfo {
                    handle,
                    layout,
                    descriptor_set,
                    vertex,
                    index,
                }
            })
        };

        // ---------------------------------------------------------------- //

        unsafe {
            let begin_info = vk::CommandBufferBeginInfo {
                flags: vk::CommandBufferUsageFlags::empty(),
                ..Default::default()
            };

            ash_device
                .begin_command_buffer(frame.commands, &begin_info)
                .unwrap();
            {
                let render_info = vk::RenderPassBeginInfo {
                    render_pass: render_pass.handle,
                    framebuffer: frame.buffer,
                    render_area: image_rect,
                    clear_value_count: 1,
                    p_clear_values: addr_of!(clear_value),
                    ..Default::default()
                };

                ash_device.cmd_begin_render_pass(
                    frame.commands,
                    &render_info,
                    vk::SubpassContents::INLINE,
                );

                for pipeline in pipelines {
                    ash_device.cmd_bind_pipeline(
                        frame.commands,
                        vk::PipelineBindPoint::GRAPHICS,
                        pipeline.handle,
                    );

                    ash_device.cmd_set_viewport(frame.commands, 0, &[viewport]);
                    ash_device.cmd_set_scissor(frame.commands, 0, &[scissor]);

                    let vertex_buffer = self.buffers.vertex.handle;
                    let vertex_offset = pipeline.vertex.offset;
                    ash_device.cmd_bind_vertex_buffers(
                        frame.commands,
                        0,
                        &[vertex_buffer],
                        &[vertex_offset],
                    );

                    if let Some(pipeline_index) = &pipeline.index {
                        let index_buffer = self.buffers.index.handle;
                        let index_offset = pipeline_index.offset;
                        ash_device.cmd_bind_index_buffer(
                            frame.commands,
                            index_buffer,
                            index_offset,
                            vk::IndexType::UINT16,
                        );
                    }

                    if let Some(descriptor_set) = pipeline.descriptor_set {
                        ash_device.cmd_bind_descriptor_sets(
                            frame.commands,
                            vk::PipelineBindPoint::GRAPHICS,
                            pipeline.layout,
                            0,
                            &[descriptor_set],
                            &[],
                        );
                    }

                    if let Some(pipeline_index) = &pipeline.index {
                        let index_count = pipeline_index.count as u32;
                        ash_device.cmd_draw_indexed(frame.commands, index_count, 1, 0, 0, 0);
                    } else {
                        let vertex_count = pipeline.vertex.count as u32;
                        ash_device.cmd_draw(frame.commands, vertex_count, 1, 0, 0);
                    }
                }
                ash_device.cmd_end_render_pass(frame.commands);
            }
            ash_device.end_command_buffer(frame.commands).unwrap();
        }

        // -------------------------------- //
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Submits the recorded rendering commands to the Vulkan Device (such as a GPU) responsible for rendering.\
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html>
    fn submit_commands(
        &self,
        frame: &Frame,
        wait_semaphores: &[vk::Semaphore],
        signal_semaphores: &[vk::Semaphore],
        signal_fence: vk::Fence,
    ) -> bool {
        let device = unsafe { self.device() };
        let ash_device = device.ash_device();

        // -------------------------------- //

        let stage_mask = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;

        let submit_info = vk::SubmitInfo {
            command_buffer_count: 1,
            p_command_buffers: addr_of!(frame.commands),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
            p_wait_dst_stage_mask: addr_of!(stage_mask),
            ..Default::default()
        };

        let res = unsafe {
            ash_device.queue_submit(device.logical.graphics_queue, &[submit_info], signal_fence)
        };

        match res {
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return false,
            _res => _res.unwrap(),
        }

        // -------------------------------- //

        true
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Presents the rendered Frame to the Window.\
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html>
    fn present_image(&self, image_idx: u32, wait_semaphores: &[vk::Semaphore]) -> bool {
        let device = unsafe { self.device() };
        let frames = &self.frames;
        let swapchain = &frames.swapchain;
        let swapchain_ext = unsafe { swapchain.ext() };

        // -------------------------------- //

        let swapchains = [swapchain.handle];

        let present_info = vk::PresentInfoKHR {
            swapchain_count: swapchains.len() as u32,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: addr_of!(image_idx),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            ..Default::default()
        };

        let res =
            unsafe { swapchain_ext.queue_present(device.logical.present_queue, &present_info) };

        let _suboptimal = match res {
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => return false,
            _res => _res.unwrap(),
        };

        if _suboptimal {
            eprintln!("SUB-PRESENT!");
        }

        true
    }
}

// ================================================================================================================================ //

impl VulkanRenderable {
    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Uploads the Texture data to the Atlas.
    pub fn texture_atlas(&self, atlas: &Atlas, texture: &Texture) {
        self.transition_atlas(
            atlas,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );

        self.stage_texture(atlas, texture);
        self.transfer_staged(atlas, texture);

        self.transition_atlas(
            atlas,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );

        self.update_descriptors(atlas);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Prepares a Command Buffer, calls the callback function, and then submits the recorded commands.
    fn single_commands(&self, callback: impl FnOnce(&ash::Device, vk::CommandBuffer)) {
        // ---------------------------------------------------------------- //

        let device = unsafe { self.device.as_ref() };
        let ash_device = device.ash_device();

        // ---------------------------------------------------------------- //

        let command_pool = self.command_pool.handle;

        let commands = self.command_pool.allocate_buffers(1);
        let _free_buffers =
            defer::defer(|| unsafe { ash_device.free_command_buffers(command_pool, &commands) });

        // ---------------------------------------------------------------- //

        let command_buffer = commands[0];

        let begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };

        unsafe { ash_device.begin_command_buffer(command_buffer, &begin_info) }.unwrap();

        // ---------------------------------------------------------------- //

        let _end_commands = defer::defer(|| {
            unsafe { ash_device.end_command_buffer(command_buffer) }.unwrap();

            let queue = device.logical.graphics_queue;

            let submit_info = vk::SubmitInfo {
                command_buffer_count: 1,
                p_command_buffers: &command_buffer,
                ..Default::default()
            };

            let res = unsafe { ash_device.queue_submit(queue, &[submit_info], vk::Fence::null()) };
            res.unwrap();

            let res = unsafe { ash_device.queue_wait_idle(queue) };
            res.unwrap();
        });

        // ---------------------------------------------------------------- //

        callback(ash_device, command_buffer);
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Transitions the Atlas Image from one Layout to another.
    pub fn transition_atlas(
        &self,
        atlas: &Atlas,
        _format: vk::Format,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
    ) {
        let (src_access, dst_access, src_stage, dst_stage) = match (old_layout, new_layout) {
            (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => {
                let src_access = vk::AccessFlags::empty();
                let dst_access = vk::AccessFlags::TRANSFER_WRITE;
                let src_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
                let dst_stage = vk::PipelineStageFlags::TRANSFER;
                (src_access, dst_access, src_stage, dst_stage)
            }
            (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => {
                let src_access = vk::AccessFlags::TRANSFER_WRITE;
                let dst_access = vk::AccessFlags::SHADER_READ;
                let src_stage = vk::PipelineStageFlags::TRANSFER;
                let dst_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
                (src_access, dst_access, src_stage, dst_stage)
            }
            _ => unimplemented!(),
        };

        let barrier = vk::ImageMemoryBarrier {
            image: atlas.image.handle,
            old_layout,
            new_layout,
            src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            src_access_mask: src_access,
            dst_access_mask: dst_access,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };

        self.single_commands(|ash_device, command_buffer| unsafe {
            ash_device.cmd_pipeline_barrier(
                command_buffer,
                src_stage,
                dst_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[barrier],
            );
        });
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Copies the Texture data to the Staging Buffer.
    fn stage_texture(&self, atlas: &Atlas, texture: &Texture) {
        let tex_size = texture.vk_device_size();
        assert!(tex_size == atlas.image.size);
        assert!(tex_size <= self.buffers.staging.size);

        {
            let mut mem = unsafe { self.buffers.memory.map(self.buffers.staging.offs, tex_size) };
            let pixels = texture.as_slice();
            let bytes = utils::bytes_ref(pixels);
            mem.copy_to(0, bytes);
        }
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Copies the Staging Buffer to the Atlas Image.
    fn transfer_staged(&self, atlas: &Atlas, texture: &Texture) {
        let tex_extent = texture.vk_extent();

        let region = vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_extent: tex_extent,
            image_offset: vk::Offset3D::default(),
            image_subresource: vk::ImageSubresourceLayers {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                mip_level: 0,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        let staging = self.buffers.staging.handle;
        let image = atlas.image.handle;

        self.single_commands(|ash_device, command_buffer| {
            unsafe {
                ash_device.cmd_copy_buffer_to_image(
                    command_buffer,
                    staging,
                    image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[region],
                )
            };
        });
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //

    /// Updates the Descriptor Sets for the Atlas Image Texture Sampler.
    fn update_descriptors(&self, atlas: &Atlas) {
        let ash_device = unsafe { self.device().ash_device() };

        let shader = &self.shaders.uv;

        let image_info = vk::DescriptorImageInfo {
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            image_view: atlas.view.handle,
            sampler: atlas.sampler.handle,
        };

        let sampler_write = vk::WriteDescriptorSet {
            dst_set: shader.descriptor_set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
            p_buffer_info: null(),
            p_image_info: &image_info,
            p_texel_buffer_view: null(),
            ..Default::default()
        };

        let descriptor_writes = [sampler_write];
        let descriptor_copies = [];

        unsafe { ash_device.update_descriptor_sets(&descriptor_writes, &descriptor_copies) };
    }

    // -------------------------------------------------------------------------------------------------------------------------------- //
}

// ================================================================================================================================ //
