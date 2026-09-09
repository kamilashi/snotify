Let me pull current links rather than reciting URLs from memory.Here's a build order rather than a topic list — each phase gets you something visible on screen, which matters because debugging all of it at once is miserable.

## Phase 1 — axum basics + static file serving

**Research:** routers, handlers, `State` extractors, `ServeDir`.

The canonical reference is axum's own static-file example, which walks through several variants side by side: serving under a path prefix, using a fallback service, and calling `ServeDir` from inside a handler.
`https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs`

Shorter version if that's too much at once — Shuttle's example is about ten lines, and note their comment that `ServeDir` falls back to serving `index.html` when a directory is requested, which is exactly the behaviour you want at `/`:
`https://docs.shuttle.dev/examples/axum-static-files.md`

**Gotcha:** `ServeDir` needs `tower-http` with the `fs` feature enabled. It's not on by default.

**Milestone:** a static HTML page served from `cargo run`.

## Phase 2 — `tokio::sync::watch`

**Research:** `channel`, `borrow_and_update`, `changed`, `send_replace`, and why `send` fails with no receivers.

Module overview, then the two type pages:
- `https://docs.rs/tokio/latest/tokio/sync/watch/index.html`
- `https://docs.rs/tokio/latest/tokio/sync/watch/fn.channel.html`
- `https://docs.rs/tokio/latest/tokio/sync/watch/struct.Receiver.html`

Read the `channel` docs carefully — the example there uses a deliberate do-while shape so the initial value gets processed before awaiting `changed()`, and the docs are explicit that only the last sent value is retained while intermediate values are dropped. Both of those are load-bearing for your design.

**Milestone:** a background task bumping a counter, printed by a second task.

## Phase 3 — SSE from a channel

This is the join between phases 1 and 2, and the least-documented part.

**Research:** `axum::response::sse`, `async_stream::stream!`, `KeepAlive`.

Start with the module docs, but note their example uses `stream::repeat_with().throttle()`, which is *not* what you want — that's a timer, not a change signal:
`https://docs.rs/axum/latest/axum/response/sse/`

Then read this discussion thread, which is exactly your question. Someone asks how to trigger the stream on their own condition rather than on an interval, and the answer is the `async_stream::try_stream!` pattern:
`https://github.com/tokio-rs/axum/discussions/1670`

A runnable repo doing the same with a channel:
`https://github.com/mouton0815/axum-sse-from-channel`

**Milestone:** `curl -N localhost:3000/events` printing a line only when state actually changes.

## Phase 4 — `EventSource` on the client

**Research:** `EventSource`, the `text/event-stream` wire format, `onmessage` vs named events, auto-reconnect.

MDN's guide is the whole thing and worth reading start to finish — it's short:
`https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events`

Interface reference, which spells out that the connection stays open until you call `close()`, and that a generic `message` event fires when there's no `event` field:
`https://developer.mozilla.org/en-US/docs/Web/API/EventSource`

A minimal working client:
`https://github.com/mdn/dom-examples/blob/main/server-sent-events/index.html`

**Pay attention to** the field format section. Consecutive `data:` lines get concatenated with newlines inserted between them, which is how you'd accidentally break JSON if you ever hand-wrote the stream. `Event::default().json_data()` handles it for you.

**Milestone:** DOM text updating live in the browser when your Rust task changes state.

## Phase 5 — WebGL2 fullscreen shader

Do this **completely separately** from phases 1–4. A static HTML file, no server involved. Mixing the two is how you end up unable to tell whether a black screen is a transport bug or a shader compile error.

**Research:** context creation, shader compile/link with error logs, VAOs, uniforms, `requestAnimationFrame`, canvas resizing and DPR.

The reference: WebGL2 Fundamentals. It's the best resource in this space by a wide margin, deliberately built from first principles rather than being recycled OpenGL material, and it makes the framing that matters — WebGL is a rasterization engine that draws points, lines and triangles from code you supply, and most of the API is just state setup for a vertex/fragment shader pair executed by `drawArrays`.
- Start: `https://webgl2fundamentals.org/webgl/lessons/webgl-fundamentals.html`
- Then: `https://webgl2fundamentals.org/webgl/lessons/webgl-shaders-and-glsl.html`
- Table of contents — the *Misc* section has "Resizing the Canvas" and "Animation", and *Tips* has "Drawing Without Data" and "Shadertoy", all directly relevant: `https://webgl2fundamentals.org/`

Note their point that VAOs are optional in WebGL1 but standard in WebGL2, so use them everywhere — arguably no downside, and the code gets simpler.

For the specific thing you need, a focused walkthrough of exactly the fullscreen-quad case with GLSL ES 3.0 and VAOs:
`https://ostefani.dev/tech-notes/webgl-drawing-full-screen-quad`

Conceptual framing of why the fullscreen quad is the fundamental primitive for this kind of work:
`https://www.cs.cornell.edu/courses/cs4620/2018sp/cs4621/lecture02/exhibit06.html`

**Milestone:** an animated gradient filling the browser window, correct on window resize.

## Phase 6 — GLSL itself

Separate skill from the WebGL plumbing, and the one you'll spend the most time on.

The Book of Shaders. Free, online, interactive, and the standard recommendation. It goes shaping functions → color → shapes → 2D matrices → patterns → randomness → noise → fractals, with editable examples in every chapter:
`https://thebookofshaders.com/`

Repo, if you want it offline: `https://github.com/patriciogonzalezvivo/thebookofshaders`

A worked example bridging Book-of-Shaders theory into real WebGL2 code:
`https://tympanus.net/codrops/2021/01/19/drawing-2d-metaballs-with-webgl2/`

## Phase 7 — transitions

**Research:** the GL Transitions convention. It's a well-established interface: a fragment shader with two `sampler2D` uniforms (`from` and `to`), a `vec2 resolution`, and a `float progress` that runs 0 → 1.

Spec and rationale: `https://gist.github.com/gre/600d8a793ca7901a25f2`
The shader collection: `https://github.com/gl-transitions/gl-transitions`
A minimal renderer, notable because it assumes you've bound a big-triangle buffer and just calls `drawArrays(gl.TRIANGLES, 0, 3)` — the same fullscreen-triangle trick: `https://npmjs.com/package/gl-transition`

**For your case specifically:** you likely don't need two textures at all. You're transitioning between two *DOM* states, not two images. A simpler design is a shader that draws a wipe/dissolve mask driven by `progress`, layered over the DOM with `mix-blend-mode`, while you swap the text at the midpoint. Read the GL Transitions shaders for effect ideas, but don't adopt the render-to-texture pipeline unless you actually want to crossfade rendered content.

## Phase 8 — display machine concerns

Only once the rest works. Search terms rather than links, since this is very setup-dependent: **browser kiosk mode**, **`requestFullscreen` and user activation**, **rAF background-tab throttling**, **disabling screen blanking / DPMS**.

## What to skip

- **WebSockets.** Now that the design is server-push-only, the entire WebSocket literature is a detour. Most SSE tutorials you'll find use `broadcast`; you want `watch`.
- **Three.js / PixiJS tutorials.** Both would teach you an abstraction you don't need for one fullscreen quad.
- **`ts-rs`, `postcard`, JSON Patch, clock-offset sync.** All were right for earlier versions of this design and are now dead weight. Skip them.
- **Anything about SPA frameworks.** A single page updating a few text nodes from an SSE handler needs no framework.

The two phases that will consume real time are 5 and 6. Phases 1–4 combined are maybe 80 lines of Rust and 15 of JavaScript, and the docs above cover them almost completely.
