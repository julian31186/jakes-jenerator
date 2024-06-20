# Build the Rust application
FROM rust:latest as builder

# Set working directory within the container
WORKDIR /app

# Copy all files from host directory to the container working dir
COPY . .

# Build program
RUN cargo build --release

# Create a minimal runtime image
FROM ubuntu

# Set working directory within the container
WORKDIR /app

# Update all packages, install rust, and install texlive in order to convert .tex -> .pdf
RUN apt-get update -y && apt-get upgrade -y && apt-get install texlive-latex-base -y && apt-get install texlive-fonts-recommended -y && apt-get install texlive-fonts-extra -y && apt-get install texlive-latex-extra -y

# Copy the built application from the builder stage
COPY --from=builder /app/target/release/jakes-jenerator /usr/local/bin/jakes-jenerator

# Copy the source files if needed (for LaTeX files, etc.)
COPY . .

# Command to run the application and convert .tex to .pdf
CMD ["sh", "-c", "/usr/local/bin/jakes-jenerator && pdflatex -output-directory=src/output src/output/jakes_resume.tex"]