import os
import json
from datetime import datetime
import time

def count_lines_in_file(file_path):
    """Count the number of lines in a given file."""
    with open(file_path, 'r', encoding='utf-8') as file:
        return sum(1 for _ in file)

def count_lines_recursively(directory):
    """Recursively count lines in files of specified types, excluding certain directories."""
    file_types = {'.py', '.tsx', '.jsx', '.ts', '.js', '.rs', '.json'}
    line_counts = {ext: 0 for ext in file_types}

    for root, dirs, files in os.walk(directory):
        # Exclude specific directories
        dirs[:] = [d for d in dirs if d not in {'node_modules', '.next', '.git', '.xbp', '.xylex', 'target', 'release'}]
        
        for file in files:
            file_ext = os.path.splitext(file)[1]
            if file_ext in file_types:
                file_path = os.path.join(root, file)
                line_counts[file_ext] += count_lines_in_file(file_path)

    return line_counts

def format_number(num):
    """Format large numbers into a more readable format."""
    if num >= 1000:
        return f"{num // 1000}k"
    return str(num)

if __name__ == "__main__":
    directory_to_check = '.'  # Change this to the directory you want to check
    result = count_lines_recursively(directory_to_check)
    total_lines = sum(result.values())
    
    # Calculate percentages
    percentages = {ext: (count / total_lines * 100) for ext, count in result.items()}
    
    today = datetime.now().strftime('%Y-%m-%d')
    unix_timestamp = int(time.time())
    
    # Create .xylex directory if it doesn't exist
    xylex_dir = '.xylex'
    if not os.path.exists(xylex_dir):
        os.makedirs(xylex_dir)
        
    # Save results as JSON
    json_data = {
        'date': today,
        'timestamp': unix_timestamp,
        'statistics': result,
        'percentages': {ext: round(pct, 2) for ext, pct in percentages.items()},
        'total_lines': total_lines
    }
    
    json_path = os.path.join(xylex_dir, 'code_stats.json')
    
    # Load existing data if file exists
    existing_data = []
    if os.path.exists(json_path):
        with open(json_path, 'r', encoding='utf-8') as f:
            try:
                existing_data = json.load(f)
                if not isinstance(existing_data, list):
                    existing_data = [existing_data]
            except json.JSONDecodeError:
                existing_data = []
    
    # Append new data
    existing_data.append(json_data)
    
    # Write updated data back to file
    with open(json_path, 'w', encoding='utf-8') as f:
        json.dump(existing_data, f, indent=2)
    
    # Write results to README.md
    with open('README.md', 'a', encoding='utf-8') as readme:
        readme.write(f'\n\n## Code Statistics ({today})\n')
        for ext, count in result.items():
            readme.write(f"{ext}: {count} lines ({percentages[ext]:.1f}%)\n")
        readme.write(f"Total: {format_number(total_lines)} lines\n")
    
    # Also print to console
    print(f"\nCode Statistics ({today})")
    for ext, count in result.items():
        print(f"{ext}: {count} lines ({percentages[ext]:.1f}%)")
    print(f"Total: {format_number(total_lines)} lines")
