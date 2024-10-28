SJC FINANCE

A Pairs Trading Algorithm in Rust for CS 128 Honors. 

Anay Joshi (anayj2), Ayush Sridhar (ayushs12), Arjun Chatterjee (arjunc4)

Project Introduction: Our goal is to create a pairs trading algorithm on stocks in the tech market. The user will input two stocks, how much money they want to invest, a floor value for loss, and a ceiling value for profit. We will output how many shares to invest in the stock they want to long and how many shares to invest in the stock they want to short based on their inputs. We have chosen to work on this project because we are interested in pairs trading and want to create something that can give recommendations to people interested in the pairs trading strategy.

Technical Overview: 
  - Technical Description: From the terminal, the user will input two stock tickers, how much money they want to invest, a floor value for loss, and a ceiling value for profit. Given the two stocks they input, we will use past data of the stock market to find which three-month period in the past matches the past three months for the two stocks by finding the period with the highest similarity. Let's call the three-month period in the past that is the most similar P1. We will then look at what happened to those stocks in the three-month period after P1, which we will call P2. Based on what happened in P2, we can determine what pairs trading strategy someone could have used to make money. This will be determined by an algorithm that has cases for whether the stock both went up, both went down, or one went up and one went down. Based on this, we can determine which stock to long and which to short. The exact values for how much to invest in each will be determined by the slope of the best-fit line of each stock's data during P2, as well as the floor and ceiling values provided by the user. Finally, these values will be outputted to terminal as recommendations to the user for what to do.
  - Checkpoint #1: Given two stocks, be able to use past data of each stock's prices to find the three-month period (P1) that is most similar to each stock's prices in the last three months. Based on the data in the three months after P1 (P2), be able to determine which stock to long and which stock to short.
  - Checkpoint #2: Create the algorithm for finding how much to invest in each stock using the techniques described above.
  - Final Submission: Improve algorithm and output results to the terminal. Potentially create a better UI if we have the time.

Possible Challenges:
  - Could be very difficult or take a long time to find a P1 that matches the inputted stocks.
  - Could be hard to implement an algorithm that covers all possible variations of the data from P1.

References:
  - Pairs Trade: Definition, How Strategy Works, and Example https://www.investopedia.com/terms/p/pairstrade.asp
  - Short Selling: Your Step-by-Step Guide for Shorting Stocks: https://www.investopedia.com/terms/s/shortselling.asp
