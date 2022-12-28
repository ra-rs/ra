#[doc = "Register `MOSCWTCR` reader"]
pub struct R(crate::R<MOSCWTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSCWTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSCWTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSCWTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSCWTCR` writer"]
pub struct W(crate::W<MOSCWTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSCWTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MOSCWTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSCWTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTS` reader - Main clock oscillator wait time setting"]
pub type MSTS_R = crate::FieldReader<u8, MSTS_A>;
#[doc = "Main clock oscillator wait time setting\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSTS_A {
    #[doc = "0: Wait time = 2 cycles (0.25 us)"]
    _0000 = 0,
    #[doc = "1: Wait time = 1024 cycles (128 us)"]
    _0001 = 1,
    #[doc = "2: Wait time = 2048 cycles (256 us)"]
    _0010 = 2,
    #[doc = "3: Wait time = 4096 cycles (512 us)"]
    _0011 = 3,
    #[doc = "4: Wait time = 8192 cycles (1024 us)"]
    _0100 = 4,
    #[doc = "5: Wait time = 16384 cycles (2048 us) (value after reset)"]
    _0101 = 5,
    #[doc = "6: Wait time = 32768 cycles (4096 us)"]
    _0110 = 6,
    #[doc = "7: Wait time = 65536 cycles (8192 us)"]
    _0111 = 7,
    #[doc = "8: Wait time = 131072 cycles (16384 us)"]
    _1000 = 8,
    #[doc = "9: Wait time = 262144 cycles (32768 us)."]
    _1001 = 9,
}
impl From<MSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTS_A) -> Self {
        variant as _
    }
}
impl MSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSTS_A> {
        match self.bits {
            0 => Some(MSTS_A::_0000),
            1 => Some(MSTS_A::_0001),
            2 => Some(MSTS_A::_0010),
            3 => Some(MSTS_A::_0011),
            4 => Some(MSTS_A::_0100),
            5 => Some(MSTS_A::_0101),
            6 => Some(MSTS_A::_0110),
            7 => Some(MSTS_A::_0111),
            8 => Some(MSTS_A::_1000),
            9 => Some(MSTS_A::_1001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == MSTS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == MSTS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == MSTS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == MSTS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == MSTS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == MSTS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == MSTS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == MSTS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == MSTS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == MSTS_A::_1001
    }
}
#[doc = "Field `MSTS` writer - Main clock oscillator wait time setting"]
pub type MSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MOSCWTCR_SPEC, u8, MSTS_A, 4, O>;
impl<'a, const O: u8> MSTS_W<'a, O> {
    #[doc = "Wait time = 2 cycles (0.25 us)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(MSTS_A::_0000)
    }
    #[doc = "Wait time = 1024 cycles (128 us)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(MSTS_A::_0001)
    }
    #[doc = "Wait time = 2048 cycles (256 us)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(MSTS_A::_0010)
    }
    #[doc = "Wait time = 4096 cycles (512 us)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(MSTS_A::_0011)
    }
    #[doc = "Wait time = 8192 cycles (1024 us)"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(MSTS_A::_0100)
    }
    #[doc = "Wait time = 16384 cycles (2048 us) (value after reset)"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(MSTS_A::_0101)
    }
    #[doc = "Wait time = 32768 cycles (4096 us)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(MSTS_A::_0110)
    }
    #[doc = "Wait time = 65536 cycles (8192 us)"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(MSTS_A::_0111)
    }
    #[doc = "Wait time = 131072 cycles (16384 us)"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(MSTS_A::_1000)
    }
    #[doc = "Wait time = 262144 cycles (32768 us)."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(MSTS_A::_1001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Main clock oscillator wait time setting"]
    #[inline(always)]
    pub fn msts(&self) -> MSTS_R {
        MSTS_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Main clock oscillator wait time setting"]
    #[inline(always)]
    #[must_use]
    pub fn msts(&mut self) -> MSTS_W<0> {
        MSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Oscillator Wait Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moscwtcr](index.html) module"]
pub struct MOSCWTCR_SPEC;
impl crate::RegisterSpec for MOSCWTCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [moscwtcr::R](R) reader structure"]
impl crate::Readable for MOSCWTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moscwtcr::W](W) writer structure"]
impl crate::Writable for MOSCWTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOSCWTCR to value 0x05"]
impl crate::Resettable for MOSCWTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
