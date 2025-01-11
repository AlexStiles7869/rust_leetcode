import sys

# Get the vecs string
vecs = sys.argv[1]

# Replace all instances of array start with correct vec! rust macro syntax
vecs_output_string = vecs.replace("[", "vec![")

# Output string
print(vecs_output_string)
