from setuptools import setup

setup(
    name="{{projectName}}",
    version="0.0.1",
    description="{{projectDescription}}",
    author="{{author}}",
    packages=["{{projectNameLower}}"],
    package_dir={"": "."},
    zip_safe=False,
    install_requires=[],
    python_requires=">=3.6",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
    ],
    include_package_data=True,
    #!startExtra "pytest"
    extras_require={
        "tests": ["pytest"],
    },
    #!endExtra
    #!startExtra "cli"
    entry_points={"console_scripts": ["{{projectNameLower}} = {{projectNameLower}}.cli:main"]},
    #!endExtra
)