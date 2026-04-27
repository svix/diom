{{/*
Expand the name of the chart.
*/}}
{{- define "diom-operator.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "diom-operator.fullname" -}}
{{- printf "%s-operator" .Release.Name | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels.
*/}}
{{- define "diom-operator.labels" -}}
helm.sh/chart: {{ printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{ include "diom-operator.selectorLabels" . }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels — stable subset used in Deployment selectors.
*/}}
{{- define "diom-operator.selectorLabels" -}}
app.kubernetes.io/name: {{ include "diom-operator.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
ServiceAccount name.
*/}}
{{- define "diom-operator.serviceAccountName" -}}
{{- if .Values.operator.serviceAccount.create }}
{{- default (include "diom-operator.fullname" .) .Values.operator.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.operator.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Container image reference.
*/}}
{{- define "diom-operator.image" -}}
{{- $tag := .Values.operator.image.tag | default .Chart.AppVersion }}
{{- printf "%s:%s" .Values.operator.image.repository $tag }}
{{- end }}
