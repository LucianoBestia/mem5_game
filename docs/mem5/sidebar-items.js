initSidebarItems({"fn":[["ordinal_numbers",""],["run","To start the Wasm application, wasm_bindgen runs this functions"],["session_storage","Get the top-level window's session storage. TODO: to save user preferences maybe?"]],"mod":[["divcardmonikermod","divcardmonikermod.rs - renders the card moniker (card name/title)"],["divfordebuggingmod","divfordebuggingmod.rs - information for debugging"],["divfullscreenmod","divfullscreenmod.rs - instruction and button for fullscreen"],["divgridcontainermod","divgridcontainermod.rs - renders the grid container with the images and most important the on click event"],["divplayeractionsmod","divplayeractionsmod.rs - renders the div to inform player what to do next and get a click action from the user"],["divplayersandscoresmod","divplayersandscoresmod.rs - renders the div that shows players and scores"],["divrulesanddescriptionmod","divrulesanddescriptionmod.rs - renders the div that shows rules and descriptions All is a static content. Great for implementing dodrio cache."],["fetchgameconfigmod","statusinviteaskbeginmod.rs - code flow from this status"],["fetchmod","fetchmod - isolate/encapsulate fetch api in a module because it is all async   ## Async world The coding style is \"strange\" because this is completely async. It is async because JavaScript (the base of wasm) is limited to one single thread. And there is a lot to do in this single thread if the thread is just waiting.   I wouldn't call this async, but \"avoid-processor-waiting\" coding.   A lot of promises (JavaScript) and futures (rust) here.   I am starting to miss good old events now.   The primary code flow starts typically in a mouse onclick event. When the async function is called, that starts a secondary code flow that is completely independent of the primary. The primary code flow will go immediately forward and will not wait for the secondary. Usually there is no need for any code after the async call in the primary. All the code now must be in the secondary code flow.   But the beginning part of the code is always the same, only the last part is different.   What about code reusing? So we must send a parameter that is a reference to a function to be executed at the end. The world is upside down now. So confusing.   ## Promises, futures, Closures, reference to functions   Once upon a time programming was single threaded. It was easy to understand how the code flows.   From the primary flow you call a function and it returns (or not) something you can use in the primary flow.   Then came multicore processors. Now multithreading makes sense. From the primary code flow you spawn a new thread (secondary flow) and do something in it. Hoping you will never need the result in the primary code flow. That can complicate things a lot, because you never know when this result can come back.   Then came JavaScript that has only one thread. No multithreading there. But there is a lot of waiting around for resources. So let invent async code on a single thread. If you wait for something you pause this code flow and other code can run in that time. After some time your code will continue as nothing happened in between. It is similar as multithread but on a single thread. And it is never, never parallel, because it is single thread.   For this to work you don't send data around any more. You send the code that should be run in the future.   And here falls down all the experience of calling functions with data. All is reversed now. The world is upside down.   You cannot use \"calling functions\" any more. You cannot pass data in a normal way.   You cannot return values in a normal way. Somebody is talking about async await syntax. I still await to see what problems will be there. ## How to call this module and have a simple life   In the primary code call the `fetch_response` function as the last instruction:   `fetch_response(&vdom_weak,&request,&call_this_function_after_fetch);`   - `vdom_weak` is the main object of dodrio virtual dom. It contains RootRenderingComponent that contains all the mutable data needed for rendering. And also the schedule_render function we need after changing the data.   - `web_sys::Request` must be prepared with url, POST, Cors, body, headers,...   - `&call_this_function_after_fetch` is the reference to a function with specific signature.  "],["gamedatamod","game data - structs and methods around game data"],["javascriptimportmod","jaascriptimport.rs - one single module to import javascript functions and objects this are examples how to call a javascript function from rust"],["localstoragemod","localstoragemod.rs - the player nickname is saved in the LocalStorage of the browser  "],["logmod","logmod.rs - logging in wasm"],["rootrenderingcomponentmod","rootrenderingcomponentmod.rs - renders the web page"],["sessionstoragemod","sessionstoragemod.rs - for debugging texts accessible everywhere  "],["statusinviteaskbeginmod","statusinviteaskbeginmod.rs - code flow from this status"],["statusinviteaskedmod","statusinviteaskedmod.rs - code flow from this status"],["statusinviteaskingmod","statusinviteaskingmod.rs - code flow from this status"],["statusplayagainmod","statusplayagainmod.rs - code flow from this status"],["statusplaybefore1stcardmod","statusplaybefore1stcardmod.rs - code flow from this status"],["statusplaybefore2ndcardmod","statusplaybefore2ndcardmod.rs - code flow from this status"],["statustaketurnbeginmod","statustaketurnbeginmod.rs - code flow from this status"],["websocketcommunicationmod","WebSocketcommunication.rs  - module that cares about WebSocket communication"],["websocketreconnectmod","websocketreconnectmod.rs - reconnection for websocket must be part of the application."]]});