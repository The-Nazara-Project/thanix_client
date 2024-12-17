#
# spec file for package specRPM_CREATION_NAME
#
# Copyright (c) 2024 SUSE LLC
#
# All modifications and additions to the file contributed by third parties
# remain the property of their copyright owners, unless otherwise agreed
# upon. The license for this file, and modifications and additions to the
# file, is the same license as for the pristine package itself (unless the
# license for the pristine package is not an Open Source License, in which
# case the license is the MIT License). An "Open Source License" is a
# license that conforms to the Open Source Definition (Version 1.9)
# published by the Open Source Initiative.

# Please submit bugfixes or comments via https://bugs.opensuse.org/
#


Name:           thanix_client
Version:        2.2.3
Release:        0.1
Summary:        Generated reference client used as standard for Nazara (github.com/The-Nazara-Project/Nazara)
# FIXME: Select a correct license from https://github.com/openSUSE/spec-cleaner#spdx-licenses
License:        MIT
# FIXME: use correct group, see "https://en.opensuse.org/openSUSE:Package_group_guidelines"
# Group:
URL:            https://github.com/The-Nazara-Project/thanix_client
Source0:        thanix_client-%{version}.tar.gz
Source1:        vendor.tar.gz
Source2:        cargo_config
BuildRequires:  git
BuildRequires:  cargo
BuildRequires:  cargo-packaging

%description

This is a generated reference client used as a dependency for Nazara (github.com/The-Nazara-Project/Nazara).
This code has been generated using Thanix (github.com/The-Nazara-Project/Thanix) using the API schema of the publicly available NetBox demo instance. (https://demo.netbox.dev)

%prep
%autosetup -p1 -a1
install -D -m 644 %{SOURCE2} .cargo/config

%build
%{cargo_build}

%install
# manually
install -D -d -m 0755 %{buildroot}%{_bindir}
install -m 0755 %{_builddir}/%{name}-%{version}/target/release/%{bin_name} %{buildroot}%{_bindir}/%{bin_name}


%files
%{_bindir}/%{bin_name}
%license LICENSE
%doc README.md
