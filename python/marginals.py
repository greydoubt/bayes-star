import json
import matplotlib.pyplot as plt

def read_and_process_file(file_path):
    data = {}
    with open(file_path, 'r') as file:
        for i, line in enumerate(file):
            json_line = json.loads(line)
            for entry in json_line['entries']:
                condition, probability = entry
                if condition not in data:
                    data[condition] = []
                data[condition].append(probability)
    return data

def plot_data(data):
    plt.figure(figsize=(10, 6))
    for condition, probabilities in data.items():
        plt.plot(probabilities, label=condition)

    plt.xlabel('Timepoint')
    plt.ylabel('Probability')
    plt.title('Probability of Conditions Over Time')
    plt.legend()
    plt.show()

def main():
    file_path = 'path_to_your_file.json'  # Replace with your file path
    data = read_and_process_file(file_path)
    plot_data(data)

if __name__ == "__main__":
    main()
