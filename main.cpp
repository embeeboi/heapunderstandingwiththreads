#include <iostream>
#include <thread>
#include "heap.h"




void code()
    {
        object *t1,*t2,*t3;
        t1 = new object(2);
        t2 = new object(4);
        t3 = new object(8);///These pointers will be in the heap
        
    }
int main()
{//Main is the stack also the main thread

    std::thread *t = new std::thread(code);//Also adding thread to the heap
    t->join();//Run it on the stack.
}