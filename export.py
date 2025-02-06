# import os
# import glob

# def generate_rs_documentation():
#     # Create output markdown file
#     with open('rust_code_documentation.md', 'w') as md_file:
#         md_file.write('# Rust Code Documentation\n\n')
        
#         # Find all .rs files in src directory recursively
#         rs_files = glob.glob('src/**/*.rs', recursive=True)
        
#         for rs_file in rs_files:
#             # Read content of each Rust file
#             with open(rs_file, 'r') as file:
#                 content = file.read()
                
#             # Write file name and content to markdown
#             md_file.write(f'## {rs_file}\n\n')
#             md_file.write('```rust\n')
#             md_file.write(content)
#             md_file.write('\n```\n\n')

# if __name__ == '__main__':
#     generate_rs_documentation()
