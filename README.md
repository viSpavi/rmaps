
# RMaps - Advanced Mind Mapping Tool written in Rust

### Whaaat, yet another mind mapping tool?
I don't like existing mind mapping tools, they just suck. Lines and nodes are just too easy to do and easy to mess up.
If you grow a mind map to a somewhat medium size, then you start encountering situations where nodes intersect with each other, forming some sort of a big messy ball of furr made of knots that are literally impossible to remove in a 2D space. A solution would be to increase dimensionality: a 3D mind map would introduce a buttload
of new UI-related issues but it would most probably postpone the "knots" issue. 
Once that bigger mind map gets big enough, you will start getting knots again, and increasing dimensionality would obviously
not work anymore (altough fascinating, 4d is not that human-compatible)

My potentially autistic and visual-loving mind requires a way to represent a lot of chaotic things in a lot of different
perspectives, yet somehow keep everything connected. 
Just to name some of them:
- university lectures
- notes regarding some books
- notes regarding research papers
- big mind map representing a project
- when I have to focus on a smaller part of my project: mind map in a project mind map
- brainstorming mindmaps when I just need a blank canvas to spit everything out
- todo lists for my day (adhd says hi)
- notes (nodes) attached to all sorts of data types (pdfs, web pages, timelines) which can be attached to everything else

I also appear to be a fanatic (but not an expert) of cognition: it's not relevant but I thought I would point that out.

Since I still have no idea about how to do all of this, I made the probably single reasonable choice of dividing everything
in different "modules" that users can use. If you don't like a module, just change it. If you want to visualize your data
differently, then just change module. If you are missing a function that you would love, then just create that module (or ask
for it to be created)
I'm really skeptical about this project because of my mediocre programming skills and the ton of stuff that have to be
implemented in order to make this project even resemble what I had in mind. 
    

### Todo:
- CTRL-A when editing text
- patch up generic_node_container
    - fix weird colors
- patch up side_panel
    - fix weird colors and layout in order to pretend it to be something cool and serious
- add links to generic_node_container
- design a tree structure to hold shortcuts
    - figure out how to run shortcuts (closures?) 

### Roadmap
- Create a somewhat "usable" mind mapping tool
- create a bunch of other modules
- work on a mindmap-integrative text editor
