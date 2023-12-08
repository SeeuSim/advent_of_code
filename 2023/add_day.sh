day=$1

# Validate day
if [[ -z $day ]]; then
  echo "A day must be specified. Usage: add_day.sh <DAY>"
  exit 1
else
  if [[ $day =~ ^([1-9]|1[0-9]|2[0-5])$ ]]; then
    # Convert string to integer for comparison
    number=$((10#$day))
    if [ $number -lt 1 ] || [ $number -gt 25 ]; then
        echo "Enter a day between 1 and 25. Entered: $number"
        exit $1
    fi
  else
    echo "Invalid Day specified: $day"
    exit 1
  fi
  echo "Adding DAY: $day"
fi

# Validate file path
echo "Creating Lib file..."
lib_file="src/day_$day.rs"



directory="$(pwd)/src"
pattern="day_*.rs"  # Change the pattern as needed
# Get the list of files matching the pattern
files=("$directory"/$pattern)
# Sort the files by modification time in descending order
sorted_files=($(ls -t -1 "${files[@]}"))
# Retrieve the latest modified file
latest_file="${sorted_files[0]}"
latest_day="0"

if [[ $latest_file =~ day_([0-9]+)\.rs ]]; then
    latest_day="${BASH_REMATCH[1]}"
    echo "The template for the latest day is: day_$latest_day."
else
    echo "No templates found. Exiting..."
    exit 1
fi

if [ -f "$lib_file" ]; then
  echo "Lib file already exists: $(pwd)/$lib_file"
else 
  cp "src/day_$latest_day.rs" "$lib_file"
  if [ -f "$lib_file" ]; then
    echo "File created successfully: $(pwd)/$lib_file"
  else
    echo "Error occurred - unable to create the file"
    exit 1
  fi
fi

echo "Creating Code Directory..."
if [ -d "src/day_${day}/" ]; then
  echo "Code directory exists: $(pwd)/day_$day/"
else
  cp -r "src/day_$latest_day/" "src/day_$day/"
  if [ -d "src/day_${day}/" ]; then
    echo "Directory created successfully: $(pwd)/src/day_$day"
  else
    rm "src/day_$day.rs"
    echo "Error occurred while creating directory"
    exit 1
  fi
fi

