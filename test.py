with open('css_properties.txt','r') as f:
    css = f.read()

css = css.split('\n')
css = [c for c in css if c != '\n']
css = [c.split(':')[0] for c in css]
css = [c.strip() for c in css]
css = [c for c in css if c != '']
css = [c for c in css if c != '\n']
print(css)

with open('css_properties.rs','w') as f:
    for c in css:
        f.write(f'''
({c.replace('-','_')}, $dict:expr, $value:expr) => {{
$dict.insert(String::from("{c}"), ($value).to_string().clone())
    }};''')
