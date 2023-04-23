# Is Rust a good first language?

<https://www.reddit.com/r/rust/comments/owmxhr> - paraphrased

Question: Is rust a good first language for a complete beginner? I have a little programming experience and I want to try low-level stuff.


## Opinion 1: your goals

If your goal is to get deep into programming then Rust is a solid choice. It will force you to recognize how things work from low to high level and why they are built the way they are. Unlike other low-level options (like C/C++) Rust also pushes you towards good practices and modern ideas/approaches which is good.

If your goal is to dabble a little then Python is great. It sweeps a lot of inconvenient details under the rug which does make it way easier to use. But the most important part is its wide ecosystem: whatever you do, from web services to games, from machine learning to video processing, there is likely a library that can help you.


## Opinion 2: learning curve

On the one hand, Rust forces you to “program well” from the start, or things simply won’t work. This steers you away from many mistakes you can make in other langues, especially other low level langues. And if you can avoid getting into lifetimes too much, the base language is very nice to work with compared to most older languages.

On the other hand, Rust forces you to think about things that some other languages handle automatically. Additionally, Rust is compiled and has less immediate ability to give you feedback outside of printing to the screen and warnings/errors. Many tools are also in 3rd party libraries, many which are still WIP, so you’ll need to learn more than just the language to do “cooler” stuff.
