/*
Copyright © 2022 Schubert Anselme <schubert@anselm.es>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

package foo

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestFooer(t *testing.T) {
	result := Fooer(3)
	if result != "Foo" {
		t.Errorf("Result was incorrect, got: %s, want: %s.", result, "Foo")
	}
}

func TestFooerTableDriven(t *testing.T) {
	// Defining the columns of the table
	tests := []struct {
		name  string
		input int
		want  string
	}{
		// the table itself
		{"9 should be Foo", 9, "Foo"},
		{"3 should be Foo", 3, "Foo"},
		{"1 is not Foo", 1, "1"},
		{"0 should be Foo", 0, "Foo"},
	}

	// The execution loop
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := Fooer(tt.input)
			if got != tt.want {
				t.Errorf("got %s, want %s", got, tt.want)
			}
		})
	}
}

func TestFooer2(t *testing.T) {
	input := 3
	result := Fooer(3)

	t.Logf("The input was %d", input)

	if result != "Foo" {
		t.Errorf("Result was incorrect, got: %s, want: %s.", result, "Foo")
	}

	t.Fatalf("Stop the test now, we have seen enough")
	t.Error("This won't be executed")
}

func TestFooerParallel(t *testing.T) {
	type args struct{ input int }

	tests := []struct {
		name string
		args args
		want string
	}{
		{"Test 3 in Parallel", args{input: 3}, "Foo"},
		{"Test 7 in Parallel", args{input: 7}, "7"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			got := Fooer(tt.args.input)
			if got != tt.want {
				t.Errorf("Result was incorrect, got %s, want %s", got, tt.want)
			}
		})
	}
}

func TestFooerSkipped(t *testing.T) {
	if testing.Short() {
		t.Skip("Skipping test in short mode.")
	}

	result := Fooer(3)
	if result != "Foo" {
		t.Errorf("Result was incorrect, got: %s, want: %s.", result, "Foo")
	}
}

func Test_With_Cleanup(t *testing.T) {
	// TODO: Some test code

	t.Cleanup(func() {
		// TODO: cleanup logic
	})

	helper(t)

	// TODO: More test code
}

func helper(t *testing.T) {
	t.Helper()
	// TODO: do something
}

func TestFooerTempDir(t *testing.T) {
	tmpDir := t.TempDir()

	// TODO: your tests
	t.Log(tmpDir)
}

func BenchmarkFooer(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Fooer(i)
	}
}

func FuzzFooer(f *testing.F) {
	f.Add(3)
	f.Fuzz(func(t *testing.T, input int) {
		Fooer(input)
	})
}

func TestFooerWithTestify(t *testing.T) {
	// assert equality
	assert.Equal(t, "Foo", Fooer(0), "0 is divisible by 3, should return Foo")

	// assert inequality
	assert.NotEqual(t, "Foo", Fooer(1), "1 is not divisible by 3, should not return Foo")
}

func TestMapWithTestify(t *testing.T) {
	// require equality
	require.Equal(t, map[int]string{1: "1", 2: "2"}, map[int]string{1: "1", 2: "3"})

	// assert equality
	assert.Equal(t, map[int]string{1: "1", 2: "2"}, map[int]string{1: "1", 2: "2"})
}
