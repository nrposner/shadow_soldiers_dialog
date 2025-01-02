# shadow_soldiers_dialog
Making a text-based game in Rust. Disco Elysium meets Papers Please. Focusing on the dialog system

I want to improve on the dialog engine I previously built with help from ChatGPT, and get more used to writing Rust from scratch. 

How should such a system work?

In the original, we have several functions and structs: 

a DialogueOption struct which which contains each option which may be presented in dialogue, along with things like possible skill challenges, items that can be picked up (just once), visibility option related to external flags, and the flags that get raised when this option is selected. 

a Dialogue struct containing a Speaker, aka the character or voice speaking, which will be linked to a portrait, the introductory dialogue, whether and what passive checks it contains, xp rewards for arriving at this dialogue, whether it is hidden (archaic, since in the original build the dialog is accessed from a room), and how much time moves forward as a result of accessing the dialogue. 

a PassiveCheck struct containing skill, target number, success and failure text, and speaker

a DialogueApp struct, which holds the dialogue-relared gamestate (and the character sheet, for some reason?)




our new design:

a struct that indicates we have entered a root-level dialogue, something we enter from the world, can't enter another from inside of itself. When we exit this, we get a special EXIT DIALOGUE message and leave to the overworld

a struct that contains the regular dialogue events, a combination of text proper, passive checks which may or may not trigger and may or may not branch into their own extended dialogues, before terminating and offering options

One enters the above struct via a Dialogue option. Options resulting from passive checks within a dialogue are treated similarly, but you don't need to select an option to enter them, they choose for you

a struct for these passive checks, more flexible to allow for branching and multiple levels stably






we would like to have a dialogue setup where dialogues can contain other dialogues, and multiple intros before getting to the response options. So, on entering a dialogue, we'd have our intro, a passive check which could, possibly on success or failure, open a new dialogue, which could itself get some responses or even contain its own passive checks, and so on. And we would need to be able to flexibly resolve these to return to the original dialogue. Should also be able to split intro dialogue around passive checks, or even split them for the sake of splitting, to keep individual pieces of text short. 

When you have this dialogue in a dialogue, how do you get back to the original? After the last relevant choice, it should return you to the flow of the root dialogue, with the assumption that there will be another intro dialogue before leading into the choices. 

so the structure Dialog : [intro, passive check, intro, passive check [intro, intro, passive check], intro] is valid, s

assume that within a single structure, passive checks don't come back to back? No, that should be possible as well, but you shouldn't have a passive check be the last thing before returning to the higher level

Once we're in a Dialog [], the order and contents are static, variation comes from internal recursion and going to other dialogues


