# Instantiate priority list
priority_list: list[str] = str.split("a b c d e f g h i j k l m n o p q r s t u v w x y z A B C D E F G H I J K L M N O P Q R S T U V W X Y Z")

# Read file and return lines in a list
def read_file(file_path: str) -> list[str]:
    file = open(file_path, 'r')
    lines: list[str] = []

    for line in file:
        lines.append(line)
    
    return lines

# Get formatted priority
def get_priority(character: str) -> int:
    return priority_list.index(character) + 1

# Get the total priority
def get_total_priority(lines: list[str]) -> int:
    total_prio = 0
    for line in lines:
        
        line = line.strip()
        i: int = int((len(line) / 2))
        first_half = line[:i]
        second_half = line[i:]

        for char in first_half:
            if first_half.count(char) and second_half.count(char) >= 1:
                first_half = first_half.replace(char, "")
               
                prio = get_priority(char)
                
                total_prio = total_prio + prio

    return total_prio

def get_badge_priority(lines: list[str]) -> int:
    total_prio = 0
    i = 1
    iter = 0
    curr_group: list[str] = []

    for line in lines:
        iter = iter + 1
        line = line.strip()
        done = False
        curr_group.append(line)
        i = i + 1

        if i < 4: continue

        for char in line:
            if done: break
            if curr_group[0].count(char) and curr_group[1].count(char) and curr_group[2].count(char) >= 1:
                print(char)
                total_prio = total_prio + get_priority(char)
                done = True

        curr_group = []
        i = 1

    return total_prio

def main():
    file_path = "input.txt";
    i = 1
    print(get_total_priority(read_file(file_path)))
    print(get_badge_priority(read_file(file_path)))

main()