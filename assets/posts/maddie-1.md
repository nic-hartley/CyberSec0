> title: Shoutout #1: Maddie Stone
> category: shoutouts
> publish: 2020-09-13
---

When Maddie Stone was named to the [2020 WIRED25][2020WIRED25] list of people "standing between us and species collapse", they wrote a little blurb about her.
It makes a pretty good starting point for any article gushing about her:

> After ­considering­ careers in interior design and with the FBI as a teenager, Stone was coaxed into pursuing a degree in engineering by her father.
> Now, as part of Project Zero, she’s been hunting the bugs hiding in Silicon ­Valley’s code.
> In the wild, these pests are known as zero-day vulnerabilities, and they can wreak havoc when exploited by hackers.

They also add that she's "climbed Mount Kilimanjaro" and "read over 80 books this year."

Of course, that's only a summary.
So let's go into some detail!

P.S.: Maddie, if you're reading this, you're too humble and need to brag _way_ more.

## Project Zero

[Project Zero], P0 for short, is a Google project to find zero-day vulnerabilities.
That is, a vuln which the developer of the software wasn't aware of, and there's no patch for yet -- in other words, which has been patched for zero days.
P0's goal is to find these 0days by looking at malware, reverse-engineering high-value targets, and more.

I'll be honest, it's hard to talk about Stone's contributions there.
She's only listed as the reporter for a single bug, but I doubt that sums up her contributions.
For one thing, in her talks about her work there, she talks about asking her teammates for help.
It seems obvious she's most likely helped her teammates in all of her areas of expertise.

For another, much more important one, in the blogpost about the bug she's listed as reporting, the introduction mentions 

Let's go over [that bug], though.
Bad Binder is a very interesting bug that allows privilege escalation once you have local access.
That might mean chaining a vulnerability in another system, or tricking the user into installing a malicious app.
Either way, it turns what might be a meaningless compromise into complete control over the device.
It was suspected to have been used by NSO Group, the notorious hackers-for-hire who've been accused repeatedly of assisting with spying on members of Citizen's Lab, Amnesty International, and a huge number of journalists.
By finding and defusing Bad Binder, there's a good chance P0 and Stone saved lives.

In the Bad Binder blogpost, P0 is very vague about how they got it, but they got some information which can be used to pinpoint the bug.
Stone breaks down how each piece of information needs informed their search for the bug, clearly and concisely.
She explains how she narrowed down the diff between the Pixel 2 and 3 kernels to one commit.
She also mentions that the bug had already been discovered by Syzbot, an Android fuzzing bot, which was why it didn't work on every device.
That section also includes an interesting note about why that bug, despite being a somewhat serious use-after-free that clearly could be exploited, didn't have a fix backported to older devices.

Finally, of course, the post has an in-depth explanation of exactly how the bug works.
It goes completely over my head, because unlike Stone, I'm not a 

## Talks

Most of [Stone's talks][talks] are about her work on Project Zero.
That makes sense: it's really cool, and who wouldn't want to talk about it?
She's done half a dozen amazing talks 
But, of course, that's hardly _all_ she talks about.

Most of the rest of her talks are about Android, one way or another.
From her talk at Kaspersky SAS 2019 about an Android botnet to 

## n00b to l33t

## Android App Reverse Engineering

  [2020WIRED25]: https://www.wired.com/story/wired25-2020-people-making-things-better
  [Project Zero]: https://googleprojectzero.blogspot.com/
  [that bug]: https://googleprojectzero.blogspot.com/2019/11/bad-binder-android-in-wild-exploit.html
  [talks]: https://ragingrock.com/pages/speaking.html
