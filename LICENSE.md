# MIT License

## Copyright & Ownership

**Project Name**: First Project - Rust Web API

**Copyright © 2026 Albert Idiatulin**

All rights reserved.

---

## License Text

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

### 1. Conditions

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

**The Software is provided "as is", without warranty of any kind, express or implied**, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the Software.

---

## 📋 What This License Means

### ✅ You CAN:

- **Use** the software for any purpose (commercial or non-commercial)
- **Copy** the software without restrictions
- **Modify** the software to suit your needs
- **Distribute** the software and your modifications
- **Sublicense** the software under the same MIT License
- **Include** the software in your projects
- **Use** private copies without attribution requirements
- **Make** profit from projects using this software
- **Use** in proprietary software projects

### ❌ You CANNOT:

- **Hold the author liable** for any damages or losses
- **Remove** copyright notice or license text
- **Claim** you wrote the original code (if you didn't)
- **Use** the author's name/logo for endorsement without permission
- **Guarantee** any specific functionality or performance

### ⚠️ You MUST:

- **Include** the original copyright notice in distributions
- **Include** a copy of the MIT License
- **Document** any modifications you make
- **Credit** the original author (recommended)
- **Acknowledge** the use of this software

---

## 📄 Full License Text

```
MIT License

Copyright (c) 2026 Albert Idiatulin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 📋 License History & Details

### License Information

| Aspect | Details |
|---|---|
| **License Name** | MIT License |
| **License Type** | Permissive Open Source License |
| **Steward** | MIT License Organization |
| **OSI Approved** | ✅ Yes |
| **Commercial Use** | ✅ Permitted |
| **Modification** | ✅ Permitted |
| **Distribution** | ✅ Permitted |
| **Private Use** | ✅ Permitted |
| **License & Copyright Notice** | ✅ Required |
| **State Changes** | ⚠️ Requires documentation |
| **Liability** | ❌ Excluded |
| **Warranty** | ❌ Excluded |
| **Sublicense** | ✅ Permitted |

### Comparison with Other Licenses

| License | Permissive | Commercial | Modification | Patent Grant | Sublicense |
|---|---|---|---|---|---|
| **MIT** | ✅ Yes | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |
| **Apache 2.0** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| **GPL v3** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes* |
| **BSD 3-Clause** | ✅ Yes | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |
| **MPL 2.0** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes* |

*GPL and similar "copyleft" licenses require derivative works to be open-sourced.

---

## 🔗 Dependency Licenses

This project depends on the following open-source libraries with their respective licenses:

### Direct Dependencies

| Library | Version | License | Source |
|---|---|---|---|
| axum | 0.8.9 | MIT | https://github.com/tokio-rs/axum |
| tokio | 1.52.1 | MIT | https://github.com/tokio-rs/tokio |
| sqlx | 0.8.6 | Apache 2.0 / MIT | https://github.com/launchbadge/sqlx |
| serde | 1.0.228 | Apache 2.0 / MIT | https://github.com/serde-rs/serde |
| bcrypt | 0.19.0 | Apache 2.0 / MIT | https://github.com/koalabear/bcrypt |
| chrono | 0.4.44 | Apache 2.0 / MIT | https://github.com/chronotope/chrono |
| tower-cookies | 0.11.0 | MIT | https://github.com/tower-rs/tower-http |
| uuid | 1.23.1 | Apache 2.0 / MIT | https://github.com/uuid-rs/uuid |
| anyhow | 1.0.102 | Apache 2.0 / MIT | https://github.com/dtolnay/anyhow |
| thiserror | 2.0.18 | Apache 2.0 / MIT | https://github.com/dtolnay/thiserror |
| async-trait | 0.1.89 | Apache 2.0 / MIT | https://github.com/dtolnay/async-trait |
| clap | 4.6.1 | Apache 2.0 / MIT | https://github.com/clap-rs/clap |
| serde_json | 1.0.149 | Apache 2.0 / MIT | https://github.com/serde-rs/json |
| dotenv | 0.15.0 | MIT | https://github.com/dotenv-rs/dotenv |

### License Compatibility

All dependencies are compatible with MIT license:
- ✅ **Permissive licenses** (MIT, Apache 2.0, BSD) - Fully compatible
- ✅ **Copyleft licenses** (GPL) - Allowed in permissive context
- ✅ **No conflicts** detected

### Transitive Dependencies

For a complete list of all transitive dependencies and their licenses:

```bash
cargo license --json
# or
cargo tree --format "{l}" --features full
```

---

## 📝 How to Apply This License

### Step 1: Include License File

Add a `LICENSE` or `LICENSE.md` file to your project root.

### Step 2: Include Copyright Notice

Add to your project README or main files:

```
Copyright (c) 2026 Albert Idiatulin
Licensed under MIT License
```

### Step 3: Add Header to Source Files (Optional but Recommended)

```rust
// MIT License
// Copyright (c) 2026 Albert Idiatulin
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy...
// See LICENSE file for full license text.
```

### Step 4: Document Modifications

If you modify the code significantly:

```rust
// Original file: src/handlers/users.rs
// Modified by: Your Name
// Modifications: Added email validation, improved error handling
// Date: 2026-05-06
```

---

## 📋 Acceptable Uses

### ✅ Examples of Permitted Uses

1. **Commercial SaaS Product**
   - Use this software in your SaaS application
   - Charge users for access
   - Keep modifications private
   - Requirement: Include license notice

2. **Open Source Project**
   - Fork and modify the code
   - Publish modifications
   - Create derivative work
   - Requirement: Include MIT license and notice

3. **Educational Institution**
   - Use in coursework and teaching
   - Distribute to students
   - Modify for educational purposes
   - Requirement: Include original license

4. **Enterprise Application**
   - Integrate into internal tools
   - Modify for your needs
   - Use in production
   - Requirement: Include license notice

5. **Embedded System**
   - Use in IoT devices
   - Create proprietary modifications
   - Sell devices commercially
   - Requirement: Include license in documentation

### ❌ Not Permitted

1. **Removing License Notice** - Must keep original copyright
2. **Claiming Authorship** - Cannot claim you wrote original code
3. **Using Author's Name for Endorsement** - Cannot use Albert Idiatulin's name for promotion
4. **Holding Author Liable** - Cannot sue for damages

---

## 🏢 Corporate Use

### Using in Commercial Projects

✅ **Allowed:**
- Include in commercial products
- Modify for internal use
- Create proprietary extensions
- Use in closed-source products
- Charge customers for your product

⚠️ **Requirements:**
- Include MIT license in distribution
- Keep copyright notice
- Provide access to license text
- Document significant changes

### Liability Clause

The software is provided "AS-IS" without warranty. The author is NOT liable for:
- Loss of data
- Loss of revenue
- Business interruption
- Personal injury
- Any indirect damages
- Any damages whatsoever

---

## 📜 Legal Interpretation

### DISCLAIMER

This document is a guide to the MIT License. For legal interpretation, consult with a lawyer.

### Key Legal Points

1. **Permissive**: You can do almost anything with the code
2. **No Warranty**: Used at your own risk
3. **No Liability**: Author cannot be sued
4. **Perpetual**: Once granted, license cannot be revoked
5. **Irrevocable**: Cannot be taken back
6. **Worldwide**: Applies in all countries
7. **Royalty-free**: No fees or royalties required

### Trademark Notice

"First Project" and related names may be trademarks of Albert Idiatulin. Use of these names does not grant trademark rights but may require permission for commercial purposes.

---

## 📞 Contact Information

### Licensing Questions

For questions about this license or its terms:

- **Email**: license@albertidiatulin.dev
- **Alternative**: albert.idiatulin@example.com
- **Website**: https://albertidiatulin.dev
- **Response Time**: 2-3 business days

### Trademark/Logo Usage

For permission to use project name or logo:

- **Email**: trademark@albertidiatulin.dev
- **Process**: Submit request with intended use

### License Change Requests

To request a dual license or alternative licensing:

- **Email**: licensing@albertidiatulin.dev
- **Required**: Detailed use case and business context

---

## 🔄 Version History

| Date | Version | Changes |
|---|---|---|
| 2026-05-06 | 1.0 | Initial MIT License adoption |

---

## 🌍 International

### Multi-Language Support

This license is written in English. For translations:
- **Disclaimer**: English version is authoritative
- **Translations**: Available for reference
- **Conflicts**: English version prevails

### Jurisdiction

This license is governed by:
- **Jurisdiction**: Massachusetts, United States
- **Law**: Massachusetts state law
- **Venue**: Massachusetts courts (if disputes arise)

---

## 🔐 License Distribution

### How to Distribute This License

**With Source Code:**
```
project-root/
├── LICENSE.md
├── README.md
├── src/
└── ...
```

**With Binary Distribution:**
```
Include in documentation or README:
"This software is licensed under MIT License.
See LICENSE.md for details."
```

**With Derivative Works:**
```
Keep original copyright notice
Add new copyright line:
"Copyright (c) 2026 Albert Idiatulin
Copyright (c) [Year] [Your Name] - Modifications"
```

---

## ✅ Checklist for Using This Project

- [ ] Read this entire license document
- [ ] Understand your permissions
- [ ] Understand limitations and disclaimers
- [ ] Include license in distributions
- [ ] Keep copyright notice intact
- [ ] Document your modifications
- [ ] Respect trademark rights
- [ ] Review dependency licenses
- [ ] Include attribution (recommended)
- [ ] Contact for questions

---

## 📚 References

- **MIT License Official**: https://opensource.org/licenses/MIT
- **OSI Approved Licenses**: https://opensource.org/licenses
- **License Selector Tool**: https://choosealicense.com
- **Software Licensing Guide**: https://tldrlegal.com/license/mit-license

---

## 🙏 Author

**Albert Idiatulin**
- Software Developer
- Open Source Enthusiast
- Project Creator

---

## 📝 Acknowledgments

This license is the standard MIT License as maintained by the Open Source Initiative (OSI) with customized attribution to Albert Idiatulin as the copyright holder.

---

**License Effective Date**: May 6, 2026

**Status**: ✅ Active

**Last Updated**: May 6, 2026

---

## ⚖️ Final Statement

This First Project is provided under the MIT License. By using this software, you agree to the terms and conditions outlined in this document. If you do not agree with these terms, please do not use this software.

For the most current version of this license, visit the project repository.

---

**Copyright © 2026 Albert Idiatulin. All rights reserved.**
