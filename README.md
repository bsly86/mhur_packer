# mhur_packer
Barebones frontend for repak that I made for modding My Hero Ultra Rumble -- all this does is let you reference a root folder and then generate a .pak file, and rename it to whatever you set the PackageName to.  

This tool is EOL, I have no desire to add more functionality (how much does it really need?) unless a wonderful idea or a terrible bug appears.

# Dependencies
[repak](https://github.com/trumank/repak) - responsible for packaging assets into a .pak file.

# Notice
Releases on this website and/or other websites include repak (in accordance with the Apache 2.0 license), for the most up to date version always visit the repository linked above. 

# Usage
Your file structure should be as follows:  
`Top Level Folder with ANY name\HerovsGame\Content\...`  
The top level folder can be referenced by name directly if it's in the same directory as the MyHeroPak executable; otherwise you must input the full path. 
