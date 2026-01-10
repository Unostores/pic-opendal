# 🖼️ pic-opendal - Easily Upload Images to the Cloud

[![Download pic-opendal](https://img.shields.io/badge/Download%20pic--opendal-latest-brightgreen)](https://github.com/Unostores/pic-opendal/releases)

## 🚀 Getting Started

Welcome to pic-opendal! This tool helps you upload images to different cloud storage services quickly and easily. You don’t need to be a tech expert to use it. Simply follow the steps below to get started.

## 📥 Download & Install

To download pic-opendal, visit this page: [Download pic-opendal](https://github.com/Unostores/pic-opendal/releases). 

1. Go to the Releases page linked above.
2. Look for the latest version of pic-opendal.
3. Click on the file that matches your operating system and it will download automatically.
4. Follow the instructions below to install it.

## 📐 Installation Steps

Once you have downloaded pic-opendal, follow these steps to install it:

1. **Open your terminal.**
2. Type the following command to install pic-opendal using Cargo:

   ```bash
   cargo install pic-od
   ```

3. Press **Enter** and wait for the installation to complete.

After installation, you can begin using pic-opendal.

## ⚙️ Configuration

To set up pic-opendal, you need to create a configuration file. This file tells pic-opendal how to work with your cloud storage accounts. Here’s how to do it:

1. Open your terminal.
2. Create a configuration file by typing the following command:

   ```bash
   nano ~/.config/pic-od/config.toml
   ```

3. In the configuration file, add the following template. Replace placeholder information with your actual details.

   ```toml
   current_profile = "default"

   [profiles.default]
   type = "s3"
   bucket = "my-bucket"
   region = "us-east-1"
   access_key_id = "YOUR_ACCESS_KEY"
   secret_access_key = "YOUR_SECRET_KEY"
   root = "/images"
   base_url = "https://cdn.example.com"
   filename_format = "{date}/{name}"

   [profiles.backup]
   type = "gcs"
   bucket = "backup-bucket"
   credential_path = "/path/to/credentials.json"
   base_url = "https://storage.googleapis.com/backup-bucket"
   ```

4. Save the file and exit the text editor.

## ☁️ Supported Storage Backends

pic-opendal supports the following cloud storage services:

- **Amazon S3**
- **Google Cloud Storage (GCS)**
- **Microsoft Azure Blob Storage**
- **Alibaba OSS**
- **Tencent COS**

This variety allows you to choose the storage option that best suits your needs. 

## 🎯 Using pic-opendal

After installation and configuration, you can start using pic-opendal. Here are the basic commands:

1. **Upload an Image**:
   Use the following command to upload an image:

   ```bash
   pic-od upload /path/to/your/image.jpg
   ```

2. **List Your Images**:
   To see what images you have uploaded, run:

   ```bash
   pic-od list
   ```

3. **Download Your Images**:
   If you need to download an image back to your local machine, use:

   ```bash
   pic-od download image_name.jpg
   ```

## 📜 Troubleshooting

If you encounter any issues, try these steps:

1. **Check Your Configuration**:
   Ensure that your configuration file has correct information.

2. **Check Connectivity**:
   Ensure that you have a stable internet connection.

3. **Inspect Error Messages**:
   Carefully read error messages for guidance on what went wrong.

## 🌟 Additional Resources

For more detailed information about pic-opendal, check the Wiki section on GitHub. The Wiki contains advanced configuration options and best practices.

For feedback or support, feel free to open an issue on the GitHub repository.

## ⚙️ Documentation

To access the full documentation, visit our repository: [Documentation](https://github.com/Unostores/pic-opendal).

## 📞 Contact

If you have any questions, you can reach out through the Issues section of this repository.

Thank you for using pic-opendal! Happy uploading!