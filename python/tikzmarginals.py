import json
import sys
import os

tikz_end = """
    \end{axis}
\end{tikzpicture}
"""

prop_order = [
    "lonely[sub=test_Jack0]",
    "exciting[sub=test_Jill0]",
    "like[obj=test_Jack9,sub=test_Jill0]",
    "like[obj=test_Jill9,sub=test_Jack0]",
    "date[obj=test_Jill9,sub=test_Jack0]",
]

legend_mapping = {
    "lonely[sub=test_Jack0]": ['red', 'triangle', 'lonely boy'],
    "exciting[sub=test_Jill0]": ['green', 'square', 'exciting girl'],
    "like[obj=test_Jack0,sub=test_Jill0]": ['blue', 'o', 'girl likes boy'],
    "like[obj=test_Jill0,sub=test_Jack0]": ['yellow', 'triangle', 'boy likes girl'],
    "date[obj=test_Jill0,sub=test_Jack0]": ['orange', 'square', 'boy dates girl'],
}

def read_tuple_list_from_file(file_path, max_lines):
    data = {}
    with open(file_path, 'r') as file:
        for i, line in enumerate(file):
            print(f"time point {i}")
            json_line = json.loads(line)
            for entry in json_line['entries']:
                print(f"entry: {entry}")
                condition, probability = entry
                if not "exist" in condition and not '{' in condition:
                    print(f"\"{condition}\" {probability}")
                    if condition not in data:
                        data[condition] = []
                    data[condition].append(probability)
    last_size = -1
    for key, value in data.items():
        if last_size == -1:
            last_size = len(value)
        else:
            assert(last_size == len(value))
    return data

def tikz_render_one_curve(prop, row):
    legend_tuple = legend_mapping[prop]
    color = legend_tuple[0]
    shape = legend_tuple[1]
    legend = legend_tuple[2]
    data_string = format_probability_vector(row)
    tikz = f"""
    \\addplot[
        color={color},
        mark={shape},
        ]
        coordinates {{
        {data_string}
        }};
        \\addlegendentry{{{legend}}}
"""
    return tikz

def format_probability_vector(probabilities):
    # Create a list of formatted strings "(index, probability)" for each probability
    formatted_pairs = [f"({index},{prob})" for index, prob in enumerate(probabilities)]

    # Join all the formatted strings into a single string
    result = ''.join(formatted_pairs)
    return result

def tikz_render_curves(data):
    buffer = ""
    # print(f"prop_order {prop_order}")
    for prop in prop_order:
        row = data[prop]
        part = tikz_render_one_curve(prop, row)
        # print(f"part: {part}")
        buffer = buffer + part
    return buffer

def create_tikz_preamble(N):
    xtick_values = ', '.join(str(i) for i in range(N))
    tix_preamble = f"""
\\begin{{tikzpicture}}
    \\begin{{axis}}[
        xlabel={{Iteration}},
        ylabel={{Marginal}},
        xmin=0, xmax={N-1},
        ymin=0, ymax=1,
        xtick={{{xtick_values}}},
        ytick={{0,0.2,0.4,0.6,0.8,1}},
        legend pos=north west,
        ymajorgrids=true,
        grid style=dashed,
    ]
"""
    return tix_preamble

def get_tuple_size(data):
    for key, value in data.items():
        return len(value)
    assert(False)

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <file_path> [max_lines]")
        sys.exit(1)
    file_path = sys.argv[1]
    max_lines = int(sys.argv[2]) if len(sys.argv) > 2 else None
    base_name = os.path.basename(file_path)
    name_without_ext = os.path.splitext(base_name)[0]
    out_path = f"./tikz_output/{name_without_ext}_{max_lines}_plot.tex"  # Modify this line as needed

    data = read_tuple_list_from_file(file_path, max_lines)

    tuple_size = get_tuple_size(data)
    preamble = create_tikz_preamble(tuple_size)
    print(preamble)
    curves = tikz_render_curves(data)
    print(curves)

    total_tikz = '\n'.join([preamble, curves, tikz_end])
    out_file = open(out_path, 'w')
    out_file.write(total_tikz)

if __name__ == "__main__":
    main()
