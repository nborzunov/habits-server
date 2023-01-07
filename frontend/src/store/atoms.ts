import { atom, selector } from 'recoil';
import { Habit } from '~/types/types';

export const habitsState = atom<Habit[]>({
    key: 'habitsState',
    default: [],
});

export const completedHabitsState = selector({
    key: 'completedHabitsState',
    get: ({ get }) => {
        const list = get(habitsState);

        return list.filter((habit) => habit.completedToday);
    },
});

const localStorageEffect =
    (key: string) =>
    ({ setSelf, onSet }: { setSelf: any; onSet: any }) => {
        const savedValue = localStorage.getItem(key);
        if (savedValue != null) {
            setSelf(JSON.parse(savedValue));
        }

        onSet((newValue: string, _: any, isReset: boolean) => {
            isReset
                ? localStorage.removeItem(key)
                : localStorage.setItem(key, JSON.stringify(newValue));
        });
    };

export const selectedHabitIdState = atom<string | null>({
    key: 'selectedHabitIdState',
    default: null,
    effects: [localStorageEffect('selectedHabit')],
});

export const selectedHabitState = selector({
    key: 'selectedHabitState',
    get: ({ get }) => {
        const list = get(habitsState);

        const selectedId = get(selectedHabitIdState);
        return list.find((habit) => habit.id == selectedId);
    },
});
