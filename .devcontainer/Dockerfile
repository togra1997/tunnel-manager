FROM python:latest

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# install pre-commit
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt --no-cache-dir

WORKDIR /app
