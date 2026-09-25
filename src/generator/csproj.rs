use crate::models::{AppSettings, PhonemizerProject, RoslynOptimizationLevel};

pub fn generate_csproj_with_settings(
    project: &PhonemizerProject,
    core_dll_path: Option<&str>,
    settings: Option<&AppSettings>,
) -> String {
    let dll_ref = if let Some(path) = core_dll_path {
        format!(
            r#" <Reference Include="OpenUtau.Core">
      <HintPath>{}</HintPath>
      <Private>false</Private>
    </Reference>"#,
            path
        )
    } else {
        r#" <PackageReference Include="OpenUtau.Core" Version="*" Condition="Exists('OpenUtau.Core.dll')" />
    <!-- Local fallback reference if in Plugins directory -->
    <Reference Include="OpenUtau.Core" Condition="Exists('../OpenUtau.Core.dll')">
      <HintPath>../OpenUtau.Core.dll</HintPath>
      <Private>false</Private>
    </Reference>
    <Reference Include="OpenUtau.Core" Condition="Exists('OpenUtau.Core.dll')">
      <HintPath>OpenUtau.Core.dll</HintPath>
      <Private>false</Private>
    </Reference>"#.to_string()
    };

    let target_framework = settings
        .map(|s| s.dotnet_target_framework.tfm_string())
        .unwrap_or("net8.0");

    let nullable = if settings.map(|s| s.nullable_context).unwrap_or(true) {
        "enable"
    } else {
        "disable"
    };

    let allow_unsafe = if settings.map(|s| s.allow_unsafe_code).unwrap_or(true) {
        "true"
    } else {
        "false"
    };

    let deterministic = if settings.map(|s| s.deterministic_build).unwrap_or(true) {
        "true"
    } else {
        "false"
    };

    let optimize = match settings.map(|s| s.roslyn_optimization).unwrap_or(RoslynOptimizationLevel::Release) {
        RoslynOptimizationLevel::Release => "true",
        RoslynOptimizationLevel::Debug => "false",
    };

    let ready_to_run = if settings.map(|s| s.ready_to_run_aot).unwrap_or(false) {
        r#" <PublishReadyToRun>true</PublishReadyToRun>
    <TieredCompilation>true</TieredCompilation>"#
    } else {
        ""
    };

    format!(
        r#"<Project Sdk="Microsoft.NET.Sdk">

  <PropertyGroup>
    <TargetFramework>{}</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>{}</Nullable>
    <AssemblyName>{}</AssemblyName>
    <RootNamespace>{}</RootNamespace>
    <Version>{}</Version>
    <Authors>{}</Authors>
    <Description>{}</Description>
    <CopyLocalLockFileAssemblies>false</CopyLocalLockFileAssemblies>
    <ProduceReferenceAssembly>false</ProduceReferenceAssembly>
    <AllowUnsafeBlocks>{}</AllowUnsafeBlocks>
    <Deterministic>{}</Deterministic>
    <Optimize>{}</Optimize>
{}
  </PropertyGroup>

  <ItemGroup>
{}
  </ItemGroup>

</Project>
"#,
        target_framework,
        nullable,
        project.output_dll_name.trim_end_matches(".dll"),
        project.namespace,
        project.version,
        project.author,
        project.description,
        allow_unsafe,
        deterministic,
        optimize,
        ready_to_run,
        dll_ref
    )
}

pub fn generate_csproj(project: &PhonemizerProject, core_dll_path: Option<&str>) -> String {
    generate_csproj_with_settings(project, core_dll_path, None)
}
