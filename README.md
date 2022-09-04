# spork

Erm... how to describe this idea.

Spork is a distributed javascript runtime (using Deno) that allows a JS script to control a workflow. Kinda?

The idea is that you can have a small javascript program that orchestrates an operation or processed some data, and when
that program becomes idle- as in it's waiting for an external operation to change or data to become available- it uploads
the program. 

The program's state is saved, and the scheduler re-scheduled execution. When the program gets re-run, the state is loaded 
and program resumed. 

```javascript

function* say_yield() {
    Deno.core.print("hello 1");
    yield;
    Deno.core.print("hello 2");
    yield;
    Deno.core.print("hello 3");
    yield;
}

var it = say_yield();
```

On first run, that program will print "hello 1", then yield. The yield will cause the program's memory will be snapshotted
and saved to the distributed storage. Then the same node, or a different node, can pick up the same program and continue 
running it. The 2nd time will print hello 2, and then go back to be re-run. 

I think this could be useful for orchestrating workflows, as you can just write a script to do the work- yielding when you're waiting
on an external operation to complete. Kinda like async programming. You do a network request, and then tell the system to do 
something else until that call returns. 



right now this is just a very rough idea, and there will be a lot of additional issues to overcome to actually make this 
functional enough to use. 


Could also be done with RustPython, if that project had any documentation...