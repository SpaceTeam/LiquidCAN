import yaml
import os

cpp_path = "./Cpp-Implementation"
enum_path = "/Infrastructure"
model_path = "/Model"


def convert_to_camel_case(value, upper=True):
    components = value.split("_")

    if not components:
        return ""

    if upper:
        return "".join(x.title() for x in components)

    res = components[0].lower()
    for part in components[1:]:
        if part:
            res += part.title()
    return res


def change_case(value, value_type):
    if value_type == "type":
        return convert_to_camel_case(value)
    elif value_type == "struct_member":
        return convert_to_camel_case(value, False)
    elif value_type == "enum_member":
        return convert_to_camel_case(value)
    return value


def add_cpp_preprocessor_directives(code_string, element_name, enum_includes=None):
    new_code_string = f"#ifndef {element_name.upper()}_H\n"
    new_code_string += f"#define {element_name.upper()}_H\n\n"
    new_code_string += "#include <cstdint>\n"

    if enum_includes:
        for inf_file in enum_includes:
            new_code_string += f"#include <..{enum_path}/{inf_file}.h>\n\n"
    else:
        new_code_string += "\n"

    new_code_string += code_string + "\n"
    new_code_string += "#endif"
    return new_code_string


def create_enum(element_name, fields, data_type=None):
    global default_types
    element_name = change_case(element_name, "type")
    code = f"enum {element_name}"

    if data_type is not None:
        if not data_type in default_types:
            raise Exception(f"Invalid data type: {data_type} in enum: {element_name}")
        code += f" : {default_types[data_type]["conversion"]["cpp"]}\n"
    else:
        code += "\n"
    code += "{\n"

    has_value = False
    for element in fields:
        code += f"\t{change_case(element["name"], "enum_member")}"

        if "value" in element:
            has_value = True
            code += f" = {element["value"]}"
        elif has_value:
            raise Exception(f"not every value in {element_name} has a value!")

        code += ",\n"
    code += "}\n"
    return add_cpp_preprocessor_directives(code, element_name)


def create_struct(element_name, fields):
    global default_types, enum_types

    inf_files = []

    element_name = change_case(element_name, "type")
    code = f"struct __attribute__((packed)) {element_name}\n"
    code += "{\n"

    required_types = []
    for element in fields:
        code += "\t"
        type_name = element["type"]
        if type_name in default_types:
            code += default_types[type_name]["conversion"]["cpp"]
        else:
            if type_name in enum_types:
                inf_files.append(change_case(type_name, "type"))
            code += f"{change_case(type_name, "type")}"
            required_types.append(type_name)

        code += f" {change_case(element["name"], "struct_member")}"
        if "length" in element:
            code += f"[{element["length"]}]"
        code += ";\n"
    code += "}\n"
    return add_cpp_preprocessor_directives(code, element_name, inf_files), required_types


def prepare_write(path):
    if os.path.exists(path):
        for dir_file in os.listdir(path):
            os.remove(f"{path}/{dir_file}")
    else:
        os.makedirs(path)


with open("./LiquidCAN.yaml", 'r') as file:
    data = yaml.safe_load(file)

type_set = set()

# default types
default_types = {}
if "dtypes" in data.keys():
    default_types = data["dtypes"]
else:
    print("No default types defined!")

# enum
cpp_enum_code = []
enum_types = set()
if "enum" in data.keys():
    for enum in data["enum"]:
        name = enum["name"]
        enum_types.add(enum["name"])
        if "type" in enum.keys():
            cpp_enum = create_enum(name, enum["values"], enum["type"])
        else:
            cpp_enum = create_enum(name, enum["values"])

        type_set.add(name)
        cpp_enum_code.append((name, cpp_enum))
else:
    print("No enum defined!")

# struct
cpp_struct_code = []
if "struct" in data.keys():
    for struct in data["struct"]:
        name = struct["name"]
        if name in enum_types:
            raise Exception(f"The name: {name} can't be used for a enum and a struct!")

        cpp_code, requires = create_struct(name, struct["fields"])
        cpp_struct_code.append((name, cpp_code, requires))
        type_set.add(name)

# Type validation
for name, cpp_struct, required_types in cpp_struct_code:
    for required_type in required_types:
        if required_type not in type_set:
            raise Exception(f"{required_type} is not defined!\n\r This type is required for {name}")

# Create cpp enums
prepare_write(f"{cpp_path}{enum_path}")
for name, code in cpp_enum_code:
    with open(f"{cpp_path}{enum_path}/{convert_to_camel_case(name)}.h", "w") as file:
        file.write(code)

# Create cpp structs
prepare_write(f"{cpp_path}{model_path}")
for name, code, _ in cpp_struct_code:
    with open(f"{cpp_path}{model_path}/{convert_to_camel_case(name)}.h", "w") as file:
        file.write(code)

