
// example: static conditions / safety guards in C to statically ensure correct data structures

struct employee
{
    char *name;

    // prevent invalid data structures:
    // always ensure that the coworker also registered 'this' as their coworker
    // this check is done at compile-time, not at runtime, so no performance is lost
    employee *coworker
        where is_coworker_of(this, coworker);
}

// return a->coworker == b || a->coworker->coworker == b || a->coworker->coworker->coworker == b || ...
bool is_coworker_of(employee *a, employee *b);

void do_work(employee *a);

int main() 
{
    // example: invalid data structure
    employee a, b;
    a.coworker = b;
    do_work(a); // this will throw a compilation error, because a is a coworker of b but b is not a coworker of a

    // example: valid data structure
    employee c, d;
    c.coworker = d;
    d.coworker = c;
    do_work(c); // compiles fine
}