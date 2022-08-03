from random import random, shuffle


class Movie:
    """
    Movie class
    """
    def __init__(self, title, submitted_by):
        self.title = title
        self.submitted_by = submitted_by
        self.score = 0

    def update_score(self, score):
        self.score += score

def get_list_of_movies():
    '''Asks users to submit movies and appends them to a list called movies'''

    movies = []

    while True:
        name = input('Whats your name?: ')
        title = input('What movie do you want to see?: ')
        movie = Movie(title, name)
        movies.append(movie)
        stop_input =  input('Is there any one else selecting a movie?(y/n): ')
        if stop_input.lower() == 'n':
            break

    return movies

def score_movies(movies):
    '''adds a score to each movie in the movies list'''
    random_movie_list = movies.copy()
    shuffle(random_movie_list)

    for movie in movies:
        print(f'\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n***{movie.submitted_by}*** please come to for a secret vote!')
        for random_movie in random_movie_list:    
            while True:
                movie_score = int(input(f'Score {random_movie.title} on a scale of 1 to 3, 1 being the worst and 3 being the best: '))
                if movie_score < 1 or movie_score > 3:
                    print('Please enter a valid score')
                else: 
                    random_movie.update_score(int(movie_score))
                    break

    return 

def tie_breaker(movies):
    '''Check if the top movies have the same score and then ask people to select only one'''
    top_movies = []
    max_score = 0

    for movie in movies:
        if max_score < movie.score:
            max_score = movie.score
    
    for movie in movies:
        if movie.score == max_score:
            top_movies.append(movie)
    

    if len(top_movies) > 1:

        
            for movie in movies:
                print('\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n ***There is a tie!***\n Time for ELIMINATION!\n')

                print(f'***{movie.submitted_by}*** please come to for a secret vote!\n==========================')
                while True:
                    user_choice = int(input(f'Select *one* movie from the following list: {[top_movie.title for top_movie in top_movies]}, if you select the 1st one than type 1, for the 2nd one type 2 and so on..: '))
                    if user_choice < 1 or user_choice > len(top_movies):
                        print('Please enter a valid choice')
                    else:
                        selected_movie = top_movies[user_choice - 1]
                        selected_movie.update_score(1)
                        break         
            tie_breaker(movies)
    else:
        print(f'The winner is: {top_movies[0].title} Submitted by: {top_movies[0].submitted_by}!')
        return

def main():
    
    movies = get_list_of_movies()
    score_movies(movies)
    tie_breaker(movies)
    
    return 

if __name__ == '__main__':
    main()