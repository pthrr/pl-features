#include <any>
#include <iostream>
#include <string>
#include <unordered_map>

struct MyClass
{
    auto print() -> void
    {
        std::cout << "MyClass prints!\n";
    }
};

struct TypeErasedMap
{
    template< typename T >
    void store( std::string const& name, T& obj )
    {
        storage[name] = &obj;
    }

    template< typename T >
    auto get( std::string const& name ) -> T&
    {
        auto iter = storage.find( name );

        if( iter != storage.end() ) {
            if( auto* ptr = std::any_cast< T* >( iter->second ) ) {
                return *ptr;
            }
        }

        throw std::runtime_error( "obj not found" );
    }

private:
    std::unordered_map< std::string, std::any > storage;
};

auto main() -> int
{
    TypeErasedMap map;
    MyClass mc;
    map.store( "MyClass", mc );
    auto& ref = map.get< MyClass >( "MyClass" );
    ref.print();
    return 0;
}
