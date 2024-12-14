import re
from scipy.optimize import linprog
import numpy as np

# Input data (replace this with actual input)
string = """
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279
"""

button_regex = re.compile(r"Button [AB]: X\+(\d+), Y\+(\d+)")
prize_regex = re.compile(r"Prize: X=(\d+), Y=(\d+)")

def process_claw_machines(data):
    claw_machines = data.strip().split("\n\n")
    total_sum = 0

    for cm in claw_machines:
        lines = cm.strip().split("\n")
        
        # Parse button and prize coordinates
        button1 = button_regex.search(lines[0])
        button2 = button_regex.search(lines[1])
        prize = prize_regex.search(lines[2])
        
        button1_x, button1_y = map(int, button1.groups())
        button2_x, button2_y = map(int, button2.groups())
        prize_x, prize_y = map(int, prize.groups())

        # Objective function: Minimize some combination (dummy for now)
        c = [3, 1]  # Coefficients for variables a and b

        # Constraints
        A_eq = [
            [button1_x, button2_x],
            [button1_y, button2_y]
        ]
        b_eq = [prize_x + 10000000000000, prize_y+ 10000000000000]

        bounds = [(0, None), (0, None)]  # Variables a, b are non-negative

        # Solve the linear problem
        result = linprog(c, A_eq=A_eq, b_eq=b_eq, bounds=bounds, integrality=1, options={'presolve':True} )

        if result.success:

            a, b = map(int, result.x)
            total_sum += 3 * a + 1 * b
        else:
            print("No solution found for:", lines)
            total_sum += 0

    return total_sum

# Process the claw machines and get the result
with open("inputs/day13") as file:

    result = process_claw_machines(file.read())
    print("Total Sum:", result)
