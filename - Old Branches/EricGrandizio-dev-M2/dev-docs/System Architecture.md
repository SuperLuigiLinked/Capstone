# System Architecture

### Wyn

The Wyn windowing library will follow an Event-Driven Architecture.

It will closely wrap the native platforms' event/message loops, presenting users of the Wyn library a single common interface regardless of platform. 

The library will provide a Rust trait `EventHandler` with a list of callback functions for each type of event.

Users of the library will be able to create their own types that `impl EventHandler`, allowing them to specify custom responses for events.

On the Main Program Thread, users will call a function to start the Event Loop, passing it an `impl EventHandler` object that the library will call into while the Event Loop runs.
(The limitation of running the Event Loop on the Main Thread is to remain portable across different platforms)

### RGE

The **R**ust-**G**ame-**E**ngine library will also follow an Event-Driven Architecture (or at least, Implicit Invocation), albeit to a much more limited extent.

Much like the Wyn library, users will run the Game Engine on the Main Thread. It will handle starting the Wyn Event Loop and creating a Main Window on its own.

Users of the library will specify Update and Render callback functions (via a Rust trait) that the Game Engine will call into at the proper times, specified as FPS (Frames-per-Second) by the user.

A proxy-object to the Game Engine will be passed into the Update/Render callbacks, allowing user-code to manipulate the Game Engine's state while it is running.
