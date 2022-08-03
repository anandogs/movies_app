# A movie scoring app

Hello! This is the first version of an app that my friends have been wanting forever.. maybe i'll create a website out of this someday. 

Code refactoring is yet to be done so please excuse the verbosity! Will add tests and refactor the rust code -- since the goal of this exercise was to build an app using rust!

Script 1 : python - "movies.py"
Script 2 : rust - the "movies" folder in this repo

- [x] Get user input from multiple user until last user is specified
- [x] Once last user has specified their choice of movie, start from first user (user name has to be taken at the time of movie prompt)
- [x] The first user is shown movies (at random, so we can't deduce who specified which movie) and they have to rate each movie in a scale of 1 - 3.. with 3 being most want to watch, 1 being least, and 2 being ambivalant
- [x] This is done till all users have inputted their values, at which point the program prints out the movie(s) with the maximum values.
- [x] In case more than 1 movie is returned, then the user is given the option for elimination - wherein each user needs to select ONE of the remaining movies
- [x] This process continues till such time 1 movie is left. That is the value maximizing movie 
