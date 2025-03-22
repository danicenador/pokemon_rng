# Pokemon RNG
Simulation of pokemon stats generation on encounter with personal id values
## Gender
- last byte of pvalue assings gender based of a threshold:
    - if 50/50, threshold is 128 and if pvalue >= threshold -> male
    - if threshold = 255, gender is unknown
