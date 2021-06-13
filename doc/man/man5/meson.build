#
# SPDX-License-Identifier: Apache-2.0
#
# Copyright (C) 2021 Shun Sakai
#

man_page = 'dsconv-config.toml.5'

man_page_source = meson.current_source_dir() / '@0@.adoc'.format(man_page)

man_page_target = custom_target(man_page,
    command : [asciidoctor,
        '-a', 'manversion=@0@'.format(meson.project_version()),
        '-b', 'manpage',
        '-D', meson.current_build_dir(),
        '@INPUT@'],
    input : man_page_source,
    install : true,
    install_dir : get_option('mandir') / 'man5',
    output : man_page)

alias_target('man5', man_page_target)
