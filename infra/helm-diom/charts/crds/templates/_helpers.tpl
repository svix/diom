{{/* Name for the upgrade job and its associated resources */}}
{{/* Name for the upgrade job and its associated resources */}}
{{- define "diom.crds.upgrade.name" -}}
{{- printf "%s-upgrade-crds" .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "diom.crds.upgrade.labels" -}}
helm.sh/chart: {{ printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/component: crds-upgrade
{{- end -}}

{{/* ServiceAccount name for the upgrade job */}}
{{- define "diom.crds.upgrade.serviceAccountName" -}}
{{- if .Values.upgrade.serviceAccount.create -}}
    {{ default (include "diom.crds.upgrade.name" .) .Values.upgrade.serviceAccount.name }}
{{- else -}}
    {{ default "default" .Values.upgrade.serviceAccount.name }}
{{- end -}}
{{- end -}}
