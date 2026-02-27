import { Tabs } from 'nextra/components'

export function CodeTabGroup({ children }) {
    return (
        <Tabs
          storageKey="programming-language"
          items={[
            'JavaScript',
            'Python',
            'Rust',
            'Go',
            'Java',
            'Kotlin',
            'Ruby',
            'C#',
            'PHP',
            'CLI',
          ]
        }>

          {children}

        </Tabs>
    );
}

export const CodeTab = Tabs.Tab;
