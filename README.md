# shadow_soldiers_dialog
Making a text-based game in Rust. Disco Elysium meets Papers Please. Focusing on the dialog system

We shall navigate between rooms using a specific area-level struct

Within rooms shall be Conversations, which shall each be connected to a separate .json file containing the relevant dialogues

Each Conversation provides a point of entry into dialogue with a separate entity, beyond which we navigate using the regular dialogue struct. It is possible to exit a Conversation from some but not all dialogues, and if a dialogue terminates without reference to another, then it must provide an exit
We must be careful to create a structure which catches infinite dialogue loops which would not allow exits. 

We must give further consideration to how the passive checks are to interact with the rest of the dialogue design. The main question: can success or failure on a passive check itself lead to branching outside of the passive checks? This remains unclear







we would like to have a dialogue setup where dialogues can contain other dialogues, and multiple intros before getting to the response options. So, on entering a dialogue, we'd have our intro, a passive check which could, possibly on success or failure, open a new dialogue, which could itself get some responses or even contain its own passive checks, and so on. And we would need to be able to flexibly resolve these to return to the original dialogue. Should also be able to split intro dialogue around passive checks, or even split them for the sake of splitting, to keep individual pieces of text short. 

When you have this dialogue in a dialogue, how do you get back to the original? After the last relevant choice, it should return you to the flow of the root dialogue, with the assumption that there will be another intro dialogue before leading into the choices. 

so the structure Dialog : [intro, passive check, intro, passive check [intro, intro, passive check], intro] is valid, s

assume that within a single structure, passive checks don't come back to back? No, that should be possible as well, but you shouldn't have a passive check be the last thing before returning to the higher level

Once we're in a Dialog [], the order and contents are static, variation comes from internal recursion and going to other dialogues


