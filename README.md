<p align="center">
<h2 align="center">ZTVRUI</h2> 
</p>
<h3 align="center">Zerotier One Controller WebUI</h3>
<br/>

## Preview

#### Network Info Page

![Networks](docs/imgs/Snipaste_2024-12-30_09-57-07.png)
</br>

<details>
<summary>Login Page</summary>

![Networks](docs/imgs/Snipaste_2024-12-30_09-56-15.png)

</summary>
</details>

<details>
<summary>Networks Page</summary>

![Networks](docs/imgs/Snipaste_2024-12-30_09-56-36.png)

</summary>
</details>

<details>
<summary>Modify Network Info Page</summary>

![Networks](docs/imgs/Snipaste_2024-12-30_09-59-30.png)

</summary>
</details>

<details>
<summary>Modify Route Page</summary>

![Networks](docs/imgs/Snipaste_2024-12-30_09-57-31.png)

</summary>
</details>

<details>
<summary>Modify IP Assignment Pool Page</summary>

![Networks](docs/imgs/Snipaste_2024-12-30_09-57-42.png)

</summary>
</details>

<details>
<summary>Modify DNS Page</summary>

![Networks](docs/imgs/Snipaste_2024-12-30_09-59-57.png)

</summary>
</details>

<details>
<summary>Modify Member Page</summary>

![Networks](docs/imgs/Snipaste_2024-12-30_10-00-07.png)

</summary>
</details>

## Installation

#### First

Download the example [configuration file](example.config.json) from the repository and modify it according to your needs.

The default username is `admin` and the default password is `password`. You can change the password by modifying the configuration file and using bcrypt to hash the new password. You can encrypt the password using [bcrypt.online](https://bcrypt.online/).

Or you can change the password by webui after the first login.

You can find the method to obtain the zerotier authtoken from the [zerotier docs](https://docs.zerotier.com/api/tokens#zerotierone-service-token).
For Linux users, you can use the following command to get the authtoken.

```bash
cat /var/lib/zerotier-one/authtoken.secret
```

**Example Configuration File**

```bash
{
  "info": {
    "username": "admin",
    "password": "$2b$08$L0G551nXjXw78mUANEC31uUXyx2SsEsmYkq7xPsa2umnQ/YSBeYV6"
  },
  "listen": "0.0.0.0:7000",
  "zerotier": {
    "auth_token": "your_zerotier_token",
    "address": "http://127.0.0.1:9993"
  }
}

```

</br>

#### Second

Download the latest release from the [release page](https://github.com/TnZzZHlp/ztvrui/releases/latest) and extract it.

Run ztvrui with the configuration file.

```bash
./ztvrui -c config.json
```
