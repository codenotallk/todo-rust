Hey. In this video I will port the todo project that I just made in C to Rust.
In my opinion. The best way to learn something new is try to get the same 
result in projects already did before.

Let's give a shot.

This id will work for now.

I think is good for now.


We already did the Task and the Task Manager. All those blocks that I made in C. Here in Rust I have a lot of things ready to use. So let's jump some blocks and implement directly.


Well. I'm not using the best approach. It takes time. All those blocks was quiclkly implemented
Easy. 

I'll add the save using serde crate

This will adjust the ID temporally


I think is good for now.


I already finished the action args. Now I'm going to implement the prompt.

In C version I used color to express error. HEre we already have a crate for that.

It was a long journey but we finished the basics.
Now we can do some improvements. Bye.

I used a simple way to do some stuff. Now I'll refactor to use the Rust resources.

These were the modification. Next will be tests and split in crates.

---------------------------------------------------------------------
Well. I just finished the basic application. Now I want to give the capacity to
choose the language that my program will use to run.

In this video I'll bring the translation module.

I already did this in C project. I'll take advantage from its implementation.

I put the default translation and I refactored the task using display trait.

I have forgot to add the translate to the menu.

Now I can load a translation file from another language. I already did in C.
I'll just take the file and paste here.

First I'll load default.
Now using the file.

Now the application can be translated to another language. That's it.

In this video I'll try to remove the storage from inside the application
That way I can choose how I'll store the data. If it will be in the file.
Or a database like SQLite.

Now if I want to user another storege I can. That's it.