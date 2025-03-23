import * as k8s from "@pulumi/kubernetes";
import { IBackendAppProps } from "../types";




export class BackendApplication {
    name: string;
    props: IBackendAppProps;
    labels: Record<string, string>;
    namespace: string;

    constructor(name: string, props: IBackendAppProps) {
        this.name = name;
        this.props = props;
        this.labels = { app: name };
        this.namespace = props.namespace;
    }

    create() {
        const deployment = new k8s.apps.v1.Deployment(this.name, {
            metadata: { name: this.name, namespace: this.namespace },
            spec: {
                selector: { matchLabels: this.labels },
                replicas: 1,
                template: {
                    metadata: { labels: this.labels },
                    spec: {
                        containers: [
                            {
                                name: this.name,
                                image: this.props.image,
                                ports: [{ containerPort: this.props.port }],
                            },
                        ],
                    },
                },
            },
        });

        const service = new k8s.core.v1.Service(this.name, {
            metadata: { name: this.name, namespace: this.namespace },
            spec: {
                type: "ClusterIP",
                selector: this.labels,
                ports: [{ port: this.props.port || 8080, targetPort: this.props.port || 8080 }],
            },
        });

        const ingress = new k8s.networking.v1.Ingress(`${this.name}-ingress`, {
            metadata: {
                name: `${this.name}-ingress`,
                namespace: this.namespace,
                annotations: {
                    "nginx.ingress.kubernetes.io/rewrite-target": "/",
                },
            },
            spec: {
                ingressClassName: "nginx",
                rules: [
                    {
                        host: this.props.domain,
                        http: {
                            paths: [
                                {
                                    path: "/",
                                    pathType: "Prefix",
                                    backend: {
                                        service: {
                                            name: service.metadata.name,
                                            port: { number: this.props.port },
                                        },
                                    },
                                },
                            ],
                        },
                    },
                ],
            },
        });
    }
}
