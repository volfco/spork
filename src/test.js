
function* say_yield() {
    Deno.core.print("hello 1");
    yield;
    Deno.core.print("hello 2");
    yield;
    Deno.core.print("hello 3");
    yield;
}

var it = say_yield();