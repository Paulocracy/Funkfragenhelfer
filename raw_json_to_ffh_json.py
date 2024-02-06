"""
This little script converts the Bundesnetzegentur JSON with the
amateur radio questions into an easier format (easier for the
Funkfragenhelfer) without sections and titles. Just compare
"resources/fragenkatalog/fragenkatalog.json" to
"resources/ffh_questions.json" to see the difference.

This script has no special dependencies and should run with any
newer Python 3 interpreter (new as of 2024). It was tested with
Python 3.9, but can probably run with any newer version, and
maybe even older versions, too.
"""

import json
from typing import Any


def json_load(path: str):
    """Loads the given JSON file and returns it as json_data (a list
    or a dictionary).

    Arguments
    ----------
    * path: str ~ The path of the JSON file
    """
    with open(path) as f:
        json_data = json.load(f)
    return json_data


def json_write(path: str, json_data: Any):
    """Writes a JSON file at the given path with the given dictionary as content.

    Arguments
    ----------
    * path: str ~  The path of the JSON file that shall be written
    * json_data: Any ~ The dictionary or list which shalll be the content of
      the created JSON file
    """
    json_output = json.dumps(json_data, indent=4)
    with open(path, "w+", encoding="utf-8") as f:
        f.write(json_output)

question_blocks = []
data = json_load("./resources/fragenkatalog/fragenkatalog.json")
sections = data["sections"]
for section_L1 in sections:
    for section_L2 in section_L1["sections"]:
        if "questions" in section_L2.keys():
            question_blocks += section_L2["questions"]
            continue
        for section_L3 in section_L2["sections"]:
            if "questions" in section_L3.keys():
                question_blocks += section_L3["questions"]
                continue
            for section_L4 in section_L3["sections"]:
                if "questions" in section_L4.keys():
                    question_blocks += section_L4["questions"]
                    continue
                for section_L5 in section_L4["sections"]:
                    if "questions" in section_L5.keys():
                        question_blocks += section_L5["questions"]
                        continue


def get_if_existing(dictionary: Any, key: str):
    if key in dictionary.keys():
        return dictionary[key]
    else:
        return ""


def get_normalized(value):
    if value is None:
        return ""
    else:
        return value

ffh_json = []
for question_block in question_blocks:
    question = {
        "category": question_block["number"][0],
        "identifier": question_block["number"],
        "question": question_block["question"],

        "answer_a": get_normalized(question_block["answer_a"]),
        "answer_b": get_normalized(question_block["answer_b"]),
        "answer_c": get_normalized(question_block["answer_c"]),
        "answer_d": get_normalized(question_block["answer_d"]),

        "picture_question": get_if_existing(question_block, "picture_question"),
        "picture_a": get_if_existing(question_block, "picture_a"),
        "picture_b": get_if_existing(question_block, "picture_b"),
        "picture_c": get_if_existing(question_block, "picture_c"),
        "picture_d": get_if_existing(question_block, "picture_d"),
    }
    ffh_json.append(question)

json_write("./resources/ffh_questions.json", ffh_json)
