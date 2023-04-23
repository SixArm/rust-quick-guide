# How do people choose which crates to use?

Rust has 100,000+ crates. How do you know which ones you should use?

Here's what developers recommend:

* Start with Blessed.rs at <https://blessed.rs>

* Consider popularity, simplicity, and examples.

* Is the project actively maintained?

* Is the API is well thought-out?  because of the contribution of a large amount of people

* How good is the documentation? How good are the examples?

* Is there a larger ecosystem of crates that depend on/extend the crate functionality

* Aim for minimalistic libraries, which do not enforce their choices on me. For example, I do not want web framework to dictate me which templating I must use.

* Prefer small and tested stuff. I look at code coverage % and LOC.

* If there are several competing products, pick the better-documented one.

* If there is huge difference in dependency LOC - for example 5M vs 1M - go with smaller one.

    I do not care if library I pick is popular or not.

* I really do not like kitchen-sink style libraries - attempting to do too much things at once, their documentation is often lacking and they need more understanding how are several their parts linked together. From my experience, I spend too much time dealing with framework itself and not writing my code.

* no-std is bonus. #webasm

* I avoid libraries without readme and without link to repository. Too much work with locating the repo and bookmarking.

* Badges in readme are plus - quick overview (LOC, Coverage, download counts, ...)

I don’t just look for recent commits to see if it’s actively maintained. I check to see if it’s more than one person. On GitHub I go to Insights > Contributors. If it’s all one person, odds are higher that the one maintainer may not always be able to work on it, and that there aren’t truly that many eyes on it.

Also the fact that the code will have suffered scrutiny by more people, which means less chance of nasty surprises in the code base.



Last commit date by itself isn't really a great maintenance indicator to me. There are times when a crate is just very stable

I think that open issues and PRs on top of last commit date is an okayish heuristic

* Review the issue tracker.

* Look at the history of past releases.

* Consider the frequency of breaking changes.

* Popularity is often, but not always, a predictor of other desirable (and some not so desirable) qualities. I grant that with deeper analysis, you can dismiss that signal in favor of others, but popularity is a decent first approximation in my experience. With that said, I think popularity brings with it certain advantages inherent to popularity itself: it puts you, the user of a popular library, in the same boat as many other people. That means that if that library becomes unmaintained, then there will probably be a defined migration path. It also means that the library maintainers are probably less likely (no guarantees) to do something that will piss off and drive away the majority of their users.

Of course, there's nothing foolproof about this. Even if you're using a popular library, you might be using a niche or uncommon part of it, and that can inhibit the advantages of being in the same boat as everyone else.

* High-quality dependents (example: if the Rust compiler depends on this particular third-party crate, it's likely to have a high quality)

* Number of dependencies, compared to library complexity.

* Prefer crates that don't use proc macros, or at least try to be very conscious about compile times. That's why I prefer nanoserde over serde for instance.

* Examples, examples, examples. So important.

Take a look at blessed.rs. It has some good suggestions for popular crates, and it’s pretty well organized.

* I look at the last update date, and at the number of users. I'd like to have at least some chances of bugfix or merging my PR, exposure to users increases the chance that someone has already solved the problem that I might encounter.

* Industry standard - Is it produced by a notable group? Or an academic organization?

* Competency 1 - Does the author have a bunch of projects on a bunch of different topics or are they all focused to one topic? (Generally a sign that they aren't knowledgeable on a single topic, specialization is bad for applying for SE jobs but good for writing actual software)

* Compentency 2 - Does the author use popular algorithms or cite blogs as reference? Are they citing other libraries algorithms? (Generally a sign that they are not informed on the state-of-the art of the problem)

* Competency 3 - Is the code clear and documented are there errors in either? (Poor documentation means the author is likely cargo-culting and even if they aren't it's harder to verify the software works as described)

* Competency 4 - What are the test cases? Are these actually the worst case? Or just random errors they happened to observe? (Pretty normal to see in testing actually, many tests are just checking for specific error inputs that the authors happened to detect rather than assessing the problem and checking for errors in general)

* Competency 5 - How much fluff is there? How plausible is the Readme? Are benchmarks returning evaluations in implausibly short times? (These are all signs that the author has no idea what's going on)

* Does the author post issues to the repository? (This shows that they are at least aware of the errors in there software instead of hiding them or even worse never bothering to look)

* Is the repository active? (This mostly applies to software that interacts with other software, something with just a libc dependency is probably not going to need much changes)


1.) Popularity

2.) I dig into the source code a bit to make sure it isn't crap.

3.) Always nice to see a version higher than 1, but good luck!

4.) Is it ergonomic? A lot of popular crates are just fuckin' weird to use. I don't like things that are a pain in the ass.

* The amount of unsafe code is also a factor. cargo geiger is a handy tool for measuring it.

Benchmarks, if available, heavily factor into my consideration.
