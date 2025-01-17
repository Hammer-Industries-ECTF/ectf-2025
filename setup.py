from setuptools import setup, find_packages

setup(
    name="2025-custom-python-tools",
    version="0.0.0",
    description="A set of python tools used for 2025 Mitre ECTF",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    author="Aidan Jacobsen",
    author_email="jacobse7@purdue.edu",
    license="",
    packages=find_packages(),
    install_requires=[
        # Add your dependencies here
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.6",
)
