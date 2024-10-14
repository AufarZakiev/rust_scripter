declare module '*.md' {
  import { ComponentOptions, Component } from 'vue';
  const VueComponent: ComponentOptions;
  const VueComponentWith: (components: Record<string, Component>) => ComponentOptions;
}