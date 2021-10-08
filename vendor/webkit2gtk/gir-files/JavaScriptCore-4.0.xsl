<xsl:stylesheet xmlns="http://www.gtk.org/introspection/core/1.0"
		xmlns:gir="http://www.gtk.org/introspection/core/1.0"
		xmlns:c="http://www.gtk.org/introspection/c/1.0"
		xmlns:glib="http://www.gtk.org/introspection/glib/1.0"
		xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
		version="1.0">
  <xsl:output method="xml" indent="yes"/>

  <!-- Insert non-glib types and additional callback types. -->
  <xsl:template match="/gir:repository/gir:namespace[@name='JavaScriptCore' and not(./gir:record[@name='GlobalContextRef'])]">
    <xsl:copy>
      <xsl:apply-templates select="@*"/>
      <xsl:text>&#xa;</xsl:text>
      <xsl:apply-templates select="document('JavaScriptCore-4.0-patch.gir')/gir:repository/gir:namespace/*"/>
      <xsl:apply-templates/>
    </xsl:copy>
  </xsl:template>

  <!-- Patch up class callback types. -->
  <xsl:template match="//gir:class[@name='Class']/gir:method[@name='add_constructor_variadic']//gir:parameter[@name='callback']/gir:type">
    <xsl:copy>
      <xsl:attribute name="name">Constructor</xsl:attribute>
      <xsl:attribute name="c:type">JSCConstructor</xsl:attribute>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="//gir:class[@name='Class']/gir:method[@name='add_method_variadic']//gir:parameter[@name='callback']/gir:type">
    <xsl:copy>
      <xsl:attribute name="name">ClassVariadicFunction</xsl:attribute>
      <xsl:attribute name="c:type">JSCClassVariadicFunction</xsl:attribute>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="//gir:class[@name='Class']/gir:method[@name='add_property']//gir:parameter[@name='getter']">
    <xsl:copy>
      <xsl:apply-templates select="@*[not(name()='scope') and not(name()='closure')]"/>
      <xsl:attribute name="scope">notified</xsl:attribute>
      <xsl:attribute name="closure">4</xsl:attribute>
      <xsl:attribute name="destroy">5</xsl:attribute>
      <xsl:text>&#xa;</xsl:text>
      <xsl:apply-templates select="*[not(name()='type')]"/>
      <xsl:text>&#xa;</xsl:text>
      <type name="PropertyGetter" c:type="JSCPropertyGetter"/>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="//gir:class[@name='Class']/gir:method[@name='add_property']//gir:parameter[@name='setter']/gir:type">
    <xsl:copy>
      <xsl:attribute name="name">PropertySetter</xsl:attribute>
      <xsl:attribute name="c:type">JSCPropertySetter</xsl:attribute>
    </xsl:copy>
  </xsl:template>

  <!-- Patch up value callback types. -->
  <xsl:template match="//gir:class[@name='Value']/gir:constructor[@name='new_function_variadic']//gir:parameter[@name='callback']/gir:type">
    <xsl:copy>
      <xsl:attribute name="name">VariadicFunction</xsl:attribute>
      <xsl:attribute name="c:type">JSCVariadicFunction</xsl:attribute>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="//gir:class[@name='Value']/gir:method[@name='object_define_property_accessor']//gir:parameter[@name='getter']">
    <xsl:copy>
      <xsl:apply-templates select="@*[not(name()='scope') and not(name()='closure')]"/>
      <xsl:attribute name="scope">notified</xsl:attribute>
      <xsl:attribute name="closure">5</xsl:attribute>
      <xsl:attribute name="destroy">6</xsl:attribute>
      <xsl:text>&#xa;</xsl:text>
      <xsl:apply-templates select="*[not(name()='type')]"/>
      <xsl:text>&#xa;</xsl:text>
      <type name="Getter" c:type="JSCGetter"/>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="//gir:class[@name='Value']/gir:method[@name='object_define_property_accessor']//gir:parameter[@name='setter']/gir:type">
    <xsl:copy>
      <xsl:attribute name="name">Setter</xsl:attribute>
      <xsl:attribute name="c:type">JSCSetter</xsl:attribute>
    </xsl:copy>
  </xsl:template>

  <!-- Identity template. -->
  <xsl:template match="@*|node()">
    <xsl:copy>
      <xsl:apply-templates select="@*|node()"/>
    </xsl:copy>
  </xsl:template>

</xsl:stylesheet>
