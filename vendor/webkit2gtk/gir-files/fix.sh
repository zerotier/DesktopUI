#!/bin/bash
set -x -e

# Remove Int32 alias because it misses c:type, it not like anyone actually cares about it.
xmlstarlet ed -L \
	-d '///_:alias[@name="Int32"]' \
	freetype2-2.0.gir

# gir uses error domain to find quark function corresponding to given error enum,
# but in this case it happens to be named differently, i.e., as g_option_error_quark.
xmlstarlet ed -L \
	-u '//*[@glib:error-domain="g-option-context-error-quark"]/@glib:error-domain' -v g-option-error-quark \
	GLib-2.0.gir

# GtkEntry icon signals incorrect assume GdkEventButton when other variants may be passed
xmlstarlet ed -L \
	-u '//_:class[@name="Entry"]/glib:signal[@name="icon-press"]//_:parameter[@name="event"]/_:type[@name="Gdk.EventButton"]/@name' -v "Gdk.Event" \
	-u '//_:class[@name="Entry"]/glib:signal[@name="icon-release"]//_:parameter[@name="event"]/_:type[@name="Gdk.EventButton"]/@name' -v "Gdk.Event" \
	Gtk-3.0.gir

# GtkIconSize usage
xmlstarlet ed -L \
	-u '//_:type[@c:type="GtkIconSize"]/@name' -v "IconSize" \
	-u '//_:type[@c:type="GtkIconSize*"]/@name' -v "IconSize" \
	Gtk-3.0.gir

# incorrect GIR due to gobject-introspection GitLab issue #189
xmlstarlet ed -L \
	-u '//_:class[@name="IconTheme"]/_:method//_:parameter[@name="icon_names"]/_:array/@c:type' -v "const gchar**" \
	-u '//_:class[@name="IconTheme"]/_:method[@name="get_search_path"]//_:parameter[@name="path"]/_:array/@c:type' -v "gchar***" \
	-u '//_:class[@name="IconTheme"]/_:method[@name="set_search_path"]//_:parameter[@name="path"]/_:array/@c:type' -v "const gchar**" \
	Gtk-3.0.gir

# incorrect GIR due to gobject-introspection GitLab issue #189
xmlstarlet ed -L \
	-u '//_:record[@name="KeyFile"]/_:method[@name="set_boolean_list"]//_:parameter[@name="list"]/_:array/@c:type' -v "gboolean*" \
	-u '//_:record[@name="KeyFile"]/_:method[@name="set_double_list"]//_:parameter[@name="list"]/_:array/@c:type' -v "gdouble*" \
	-u '//_:record[@name="KeyFile"]/_:method[@name="set_integer_list"]//_:parameter[@name="list"]/_:array/@c:type' -v "gint*" \
	-u '//_:record[@name="KeyFile"]/_:method[@name="set_locale_string_list"]//_:parameter[@name="list"]/_:array/@c:type' -v "const gchar* const*" \
	-u '//_:record[@name="KeyFile"]/_:method[@name="set_string_list"]//_:parameter[@name="list"]/_:array/@c:type' -v "const gchar* const*" \
	GLib-2.0.gir

# incorrect GIR due to gobject-introspection GitLab issue #189
xmlstarlet ed -L \
	-u '//_:class[@name="Object"]/_:method[@name="getv"]//_:parameter[@name="names"]/_:array/@c:type' -v "const gchar**" \
	-u '//_:class[@name="Object"]/_:method[@name="getv"]//_:parameter[@name="values"]/_:array/@c:type' -v "GValue*" \
	-u '//_:class[@name="Object"]/_:method[@name="setv"]//_:parameter[@name="names"]/_:array/@c:type' -v "const gchar**" \
	-u '//_:class[@name="Object"]/_:method[@name="setv"]//_:parameter[@name="values"]/_:array/@c:type' -v "const GValue*" \
	-u '//_:class[@name="Object"]/_:constructor[@name="new_with_properties"]//_:parameter[@name="names"]/_:array/@c:type' -v "const char**" \
	-u '//_:class[@name="Object"]/_:constructor[@name="new_with_properties"]//_:parameter[@name="values"]/_:array/@c:type' -v "const GValue*" \
	GObject-2.0.gir

# fix wrong "full" transfer ownership
xmlstarlet ed -L \
	-u '//_:method[@c:identifier="gdk_frame_clock_get_current_timings"]/_:return-value/@transfer-ownership' -v "none" \
	-u '//_:method[@c:identifier="gdk_frame_clock_get_timings"]/_:return-value/@transfer-ownership' -v "none" \
	Gdk-3.0.gir

# replace gmodule with gpointer
xmlstarlet ed -L \
	-u '//_:record[@name="PixbufModule"]/_:field[@name="module"]/_:type/@name' -v "gpointer" \
	-u '//_:record[@name="PixbufModule"]/_:field[@name="module"]/_:type/@c:type' -v "gpointer" \
	GdkPixbuf-2.0.gir

# replace "gint" response_id parameters with "ResponseType"
xmlstarlet ed -L \
	-u '//_:parameter[@name="response_id"]/_:type[@name="gint"]/@c:type' -v "GtkResponseType" \
	-u '//_:parameter[@name="response_id"]/_:type[@name="gint"]/@name' -v "ResponseType" \
	Gtk-3.0.gir Gtk-4.0.gir

# fix wrong "full" transfer ownership
xmlstarlet ed -L \
	-u '//_:constructor[@c:identifier="gtk_shortcut_label_new"]/_:return-value/@transfer-ownership' -v "none" \
	Gtk-3.0.gir

# add out annotation for functions returning GValue
xmlstarlet ed -L \
	-a '//_:method[@c:identifier="gtk_style_context_get_style_property"]//_:parameter[@name="value" and not(@direction)]' -type attr -n "direction" -v "out" \
	-a '//_:method[@c:identifier="gtk_style_context_get_style_property"]//_:parameter[@name="value" and not(@caller-allocates)]' -type attr -n "caller-allocates" -v "1" \
	-a '//_:method[@c:identifier="gtk_cell_area_cell_get_property"]//_:parameter[@name="value" and not(@direction)]' -type attr -n "direction" -v "out" \
	-a '//_:method[@c:identifier="gtk_cell_area_cell_get_property"]//_:parameter[@name="value" and not(@caller-allocates)]' -type attr -n "caller-allocates" -v "1" \
	-a '//_:method[@c:identifier="gtk_container_child_get_property"]//_:parameter[@name="value" and not(@direction)]' -type attr -n "direction" -v "out" \
	-a '//_:method[@c:identifier="gtk_container_child_get_property"]//_:parameter[@name="value" and not(@caller-allocates)]' -type attr -n "caller-allocates" -v "1" \
	-a '//_:method[@c:identifier="gtk_widget_style_get_property"]//_:parameter[@name="value" and not(@direction)]' -type attr -n "direction" -v "out" \
	-a '//_:method[@c:identifier="gtk_widget_style_get_property"]//_:parameter[@name="value" and not(@caller-allocates)]' -type attr -n "caller-allocates" -v "1" \
	Gtk-3.0.gir

xmlstarlet tr JavaScriptCore-4.0.xsl JavaScriptCore-4.0.gir | xmlstarlet fo >JavaScriptCore-4.0.gir.tmp
mv JavaScriptCore-4.0.gir.tmp JavaScriptCore-4.0.gir

# fill in types from JavaScriptCore
xmlstarlet ed -L \
	-i '///_:type[not(@name) and @c:type="JSGlobalContextRef"]' -t 'attr' -n 'name' -v "JavaScriptCore.GlobalContextRef" \
	-i '///_:type[not(@name) and @c:type="JSValueRef"]' -t 'attr' -n 'name' -v "JavaScriptCore.ValueRef" \
	WebKit2WebExtension-4.0.gir WebKit2-4.0.gir

xmlstarlet ed -L \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type/@name' -v "guint" \
	-u '//_:constant[@name="DOM_NODE_FILTER_SHOW_ALL"]/_:type/@c:type' -v "guint" \
	WebKit2WebExtension-4.0.gir

# remove freetype and graphite methods; GitHub issue #2557
xmlstarlet ed -L \
	-d '///_:function[@c:identifier="hb_graphite2_face_get_gr_face"]' \
	-d '///_:function[@c:identifier="hb_graphite2_font_get_gr_font"]' \
	-d '///_:function[@c:identifier="hb_ft_face_create"]' \
	-d '///_:function[@c:identifier="hb_ft_face_create_cached"]' \
	-d '///_:function[@c:identifier="hb_ft_face_create_referenced"]' \
	-d '///_:function[@c:identifier="hb_ft_font_create"]' \
	-d '///_:function[@c:identifier="hb_ft_font_create_cached"]' \
	-d '///_:function[@c:identifier="hb_ft_font_create_referenced"]' \
	-d '///_:function[@c:identifier="hb_ft_font_get_face"]' \
	-d '///_:function[@c:identifier="hb_ft_font_lock_face"]' \
	HarfBuzz-0.0.gir

# fix harfbuzz types on Pango
xmlstarlet ed -L \
	-i '///_:type[not(@name) and @c:type="hb_font_t*"]' -t 'attr' -n 'name' -v "gconstpointer" \
	-u '//_:type[@c:type="hb_font_t*"]/@c:type' -v "gconstpointer" \
	-i '///_:array[not(@name) and @c:type="hb_feature_t*"]' -t 'attr' -n 'name' -v "gconstpointer" \
	-r '///_:array[@c:type="hb_feature_t*"]' -v "type" \
	-d '//_:type[@c:type="hb_feature_t*"]/*' \
	-d '//_:type[@c:type="hb_feature_t*"]/@length' \
	-d '//_:type[@c:type="hb_feature_t*"]/@zero-terminated' \
	-u '//_:type[@c:type="hb_feature_t*"]/@c:type' -v "gconstpointer" \
	Pango-1.0.gir

#  Remove unstable method from focal release
xmlstarlet ed -L \
	-d '///_:method[@c:identifier="atk_plug_set_child"]' \
	Atk-1.0.gir

# fix non-existant c-types
xmlstarlet ed -L \
	-u '//_:class[@name="WaylandDevice"]/_:method[@name="get_wl_keyboard"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	-u '//_:class[@name="WaylandDevice"]/_:method[@name="get_wl_pointer"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	-u '//_:class[@name="WaylandDevice"]/_:method[@name="get_wl_seat"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	-u '//_:class[@name="WaylandDisplay"]/_:method[@name="get_wl_compositor"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	-u '//_:class[@name="WaylandDisplay"]/_:method[@name="get_wl_display"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	-u '//_:class[@name="WaylandMonitor"]/_:method[@name="get_wl_output"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	-u '//_:class[@name="WaylandSeat"]/_:method[@name="get_wl_seat"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	-u '//_:class[@name="WaylandSurface"]/_:method[@name="get_wl_surface"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
	GdkWayland-4.0.gir

# Fix invalid type for GtkImage and GtkStackSwitcher "icon-size" property
xmlstarlet ed -L \
	-u '//_:class[@name="Image"]/_:property[@name="icon-size"]/_:type/@c:type' -v "GtkIconSize" \
	-u '//_:class[@name="Image"]/_:property[@name="icon-size"]/_:type/@name' -v "IconSize" \
	-u '//_:class[@name="StackSwitcher"]/_:property[@name="icon-size"]/_:type/@c:type' -v "GtkIconSize" \
	-u '//_:class[@name="StackSwitcher"]/_:property[@name="icon-size"]/_:type/@name' -v "IconSize" \
	Gtk-3.0.gir
