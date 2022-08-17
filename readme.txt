
a recursive macro preprocessor,

which can be used to quickly define new syntaxes to any source language and
                  as a static code-analysis tool.
                
source files are simply piped through m5 like this:
  $ cd examples
  $ cat glVertexAttribPointer.js.m5 | m5
  $ cat glsl.m5 | m5
  $ cat asm.m5 | m5
  $ cat script.sh.m5 | m5 | sh
  
Another example of its usage: github.com/Merlin-Brandt/Cpp--Wasm--OpenGL--m5
