import yaml
import os

cpp_path = "./Cpp-Implementation"
enum_path = "/Infrastructure"
model_path = "/Model"


type_converter_dict_cpp = {
    "uint8": "uint8_t",
    "uint16": "uint16_t",
    "uint32": "uint32_t",
    "int8": "int8_t",
    "int16": "int16_t",
    "int32": "int32_t",
    "float": "float_t",
    "char": "int8_t"
}


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


def add_cpp_preprocessor_directives(code_string, name):
    new_code_string = f"#ifndef {name.upper()}_H\n"
    new_code_string += f"#define {name.upper()}_H\n\n"
    new_code_string += "#include <cstdint>\n\n"
    new_code_string += code_string + "\n"
    new_code_string += "#endif"
    return new_code_string


def create_enum(name, fields, data_type=None):
    name = change_case(name, "type")
    code = f"enum {name}"

    if data_type is not None:
        if not data_type in type_converter_dict_cpp:
            raise Exception(f"Invalid data type: {data_type}")
        code += f" : {type_converter_dict_cpp[data_type]}\n"
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
            raise Exception(f"not every value in {name} has a value!")

        code += ",\n"
    code += "}\n"
    return add_cpp_preprocessor_directives(code, name)


def create_struct(name, fields, language):
    name = change_case(name, "type")
    code = f"struct {name}\n"
    code += "{\n"
    required_types = []
    for element in fields:
        code += "\t"

        type_name = element["type"]
        if type_name in type_converter_dict_cpp:
            code += type_converter_dict_cpp[type_name]
        else:
            code += f"{change_case(type_name, "type")}"
            required_types.append(type_name)

        code += f" {change_case(element["name"], "struct_member")}"
        if "length" in element:
            code += f"[{element["length"]}]"
        code += ";\n"
    code += "}\n"
    return add_cpp_preprocessor_directives(code, name), required_types


def prepare_write(path):
    if os.path.exists(path):
        for dir_file in os.listdir(path):
            os.remove(f"{path}/{dir_file}")
    else:
        os.makedirs(path)


with open("./LiquidCAN.yaml", 'r') as file:
    data = yaml.safe_load(file)

type_set = set()

# enum
cpp_enum_code = []
if "enum" in data.keys():
    for enum in data["enum"]:
        name = enum["name"]
        if "type" in enum.keys():
            cpp_enum = create_enum(name, enum["values"], enum["type"])
        else:
            cpp_enum = data = create_enum(name, enum["values"])

        type_set.add(name)
        cpp_enum_code.append((name, cpp_enum))

# struct
cpp_struct_code = []
if "struct" in data.keys():
    for struct in data["struct"]:
        name = struct["name"]
        cpp_code, requires = create_struct(name, struct["fields"], "cpp")
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

