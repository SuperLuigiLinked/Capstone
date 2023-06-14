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
            &self.surface,
            &self.ds_info,
            &self.pipelines.render_pass,
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

    /// Records the actual commands to render the Frame into a Command Buffer.
    fn record_commands(&self, frame: &Frame, settings: &RenderSettings) {
        let device = unsafe { self.device() };
        let ash_device = device.ash_device();

        let ds_info = &self.ds_info;
        let surface = &self.surface;
        let render_pass = &self.pipelines.render_pass;
        let pipeline_layout = &self.pipelines.layout;

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
                    1.0,
                ],
            },
        };

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

            /// Vertex Buffer.
            vertex: SubBuffer,

            /// Index Buffer.
            index: Option<SubBuffer>,
        }

        // ---------------------------------------------------------------- //

        let pipelines = {
            let bytes_point_list = bytes_ref(&settings.point_list);
            let bytes_line_list = bytes_ref(&settings.line_list);
            let bytes_tri_list = bytes_ref(&settings.tri_list);
            let bytes_line_strip = bytes_ref(&settings.line_strip);
            let bytes_tri_strip = bytes_ref(&settings.tri_strip);
            let bytes_tri_fan = bytes_ref(&settings.tri_fan);
            let bytes_line_strip_idx = bytes_ref(&settings.line_strip_idx);
            let bytes_tri_strip_idx = bytes_ref(&settings.tri_strip_idx);
            let bytes_tri_fan_idx = bytes_ref(&settings.tri_fan_idx);

            let bytes_vtx = bytes_point_list.len()
                + bytes_line_list.len()
                + bytes_tri_list.len()
                + bytes_line_strip.len()
                + bytes_tri_strip.len()
                + bytes_tri_fan.len();

            let bytes_idx =
                bytes_line_strip_idx.len() + bytes_tri_strip_idx.len() + bytes_tri_fan_idx.len();

            // ---------------------------------------------------------------- //

            let push_bytes = |buffer: &mut [u8], offset: usize, bytes: &[u8]| -> usize {
                let new_offset = offset + bytes.len();
                let mem_slice = &mut buffer[offset..new_offset];
                mem_slice.copy_from_slice(bytes);
                new_offset
            };

            // ---------------------------------------------------------------- //

            let mut vertex_map = unsafe {
                self.buffers
                    .memory
                    .map(self.buffers.vertex.offs, bytes_vtx as vk::DeviceSize)
            };
            let buffer_vertex = vertex_map.bytes_mut();

            let offset_point_list = 0;
            let offset_line_list = push_bytes(buffer_vertex, offset_point_list, bytes_point_list);
            let offset_tri_list = push_bytes(buffer_vertex, offset_line_list, bytes_line_list);
            let offset_line_strip = push_bytes(buffer_vertex, offset_tri_list, bytes_tri_list);
            let offset_tri_strip = push_bytes(buffer_vertex, offset_line_strip, bytes_line_strip);
            let offset_tri_fan = push_bytes(buffer_vertex, offset_tri_strip, bytes_tri_strip);
            let _offset_vtx = push_bytes(buffer_vertex, offset_tri_fan, bytes_tri_fan);

            drop(vertex_map);

            // ---------------------------------------------------------------- //

            let mut index_map = unsafe {
                self.buffers
                    .memory
                    .map(self.buffers.index.offs, bytes_idx as vk::DeviceSize)
            };
            let buffer_index = index_map.bytes_mut();

            let offset_line_strip_idx = 0;
            let offset_tri_strip_idx =
                push_bytes(buffer_index, offset_line_strip_idx, bytes_line_strip_idx);
            let offset_tri_fan_idx =
                push_bytes(buffer_index, offset_tri_strip_idx, bytes_tri_strip_idx);
            let _offset_idx = push_bytes(buffer_index, offset_tri_fan_idx, bytes_tri_fan_idx);

            drop(index_map);

            // ---------------------------------------------------------------- //

            let handles = &self.pipelines.graphics.handles;

            let info_point_list = PipelineInfo {
                handle: handles[0],
                vertex: SubBuffer {
                    offset: offset_point_list as vk::DeviceSize,
                    count: settings.point_list.len() as vk::DeviceSize,
                },
                index: None,
            };
            let info_line_list = PipelineInfo {
                handle: handles[1],
                vertex: SubBuffer {
                    offset: offset_line_list as vk::DeviceSize,
                    count: settings.line_list.len() as vk::DeviceSize,
                },
                index: None,
            };
            let info_tri_list = PipelineInfo {
                handle: handles[2],
                vertex: SubBuffer {
                    offset: offset_tri_list as vk::DeviceSize,
                    count: settings.tri_list.len() as vk::DeviceSize,
                },
                index: None,
            };
            let info_line_strip = PipelineInfo {
                handle: handles[3],
                vertex: SubBuffer {
                    offset: offset_line_strip as vk::DeviceSize,
                    count: settings.line_strip.len() as vk::DeviceSize,
                },
                index: Some(SubBuffer {
                    offset: offset_line_strip_idx as vk::DeviceSize,
                    count: settings.line_strip_idx.len() as vk::DeviceSize,
                }),
            };
            let info_tri_strip = PipelineInfo {
                handle: handles[4],
                vertex: SubBuffer {
                    offset: offset_tri_strip as vk::DeviceSize,
                    count: settings.tri_strip.len() as vk::DeviceSize,
                },
                index: Some(SubBuffer {
                    offset: offset_tri_strip_idx as vk::DeviceSize,
                    count: settings.tri_strip_idx.len() as vk::DeviceSize,
                }),
            };
            let info_tri_fan = PipelineInfo {
                handle: handles[5],
                vertex: SubBuffer {
                    offset: offset_tri_fan as vk::DeviceSize,
                    count: settings.tri_fan.len() as vk::DeviceSize,
                },
                index: Some(SubBuffer {
                    offset: offset_tri_fan_idx as vk::DeviceSize,
                    count: settings.tri_fan_idx.len() as vk::DeviceSize,
                }),
            };

            [
                info_tri_fan,
                info_tri_strip,
                info_tri_list,
                info_line_strip,
                info_line_list,
                info_point_list,
            ]
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

                    let rotate_pc = (settings.updates as f64 / settings.fps) as f32;

                    let pc = glsl::PushConstants { rotate_pc };

                    ash_device.cmd_push_constants(
                        frame.commands,
                        pipeline_layout.handle,
                        pipeline_layout.pc_range.stage_flags,
                        pipeline_layout.pc_range.offset,
                        pc.as_bytes(),
                    );

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

    // -------------------------------------------------------------------------------------------------------------------------------- //
}

// ================================================================================================================================ //
