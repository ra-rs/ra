#[doc = "Register `SFMSPC` reader"]
pub struct R(crate::R<SFMSPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSPC` writer"]
pub struct W(crate::W<SFMSPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSPC_SPEC>;
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
impl From<crate::W<SFMSPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMSPI` reader - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately."]
pub type SFMSPI_R = crate::FieldReader<u8, SFMSPI_A>;
#[doc = "Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMSPI_A {
    #[doc = "0: Extended SPI protocol"]
    _00 = 0,
    #[doc = "1: Dual SPI protocol"]
    _01 = 1,
    #[doc = "2: Quad SPI protocol"]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<SFMSPI_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMSPI_A) -> Self {
        variant as _
    }
}
impl SFMSPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSPI_A {
        match self.bits {
            0 => SFMSPI_A::_00,
            1 => SFMSPI_A::_01,
            2 => SFMSPI_A::_10,
            3 => SFMSPI_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SFMSPI_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SFMSPI_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SFMSPI_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SFMSPI_A::_11
    }
}
#[doc = "Field `SFMSPI` writer - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately."]
pub type SFMSPI_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SFMSPC_SPEC, u8, SFMSPI_A, 2, O>;
impl<'a, const O: u8> SFMSPI_W<'a, O> {
    #[doc = "Extended SPI protocol"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SFMSPI_A::_00)
    }
    #[doc = "Dual SPI protocol"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SFMSPI_A::_01)
    }
    #[doc = "Quad SPI protocol"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SFMSPI_A::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SFMSPI_A::_11)
    }
}
#[doc = "Field `SFMSDE` reader - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected."]
pub type SFMSDE_R = crate::BitReader<SFMSDE_A>;
#[doc = "Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFMSDE_A {
    #[doc = "0: Does not allocate minimum switch time"]
    _0 = 0,
    #[doc = "1: Allocate the minimum switch time equivalent to 1*QSPXLK"]
    _1 = 1,
}
impl From<SFMSDE_A> for bool {
    #[inline(always)]
    fn from(variant: SFMSDE_A) -> Self {
        variant as u8 != 0
    }
}
impl SFMSDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMSDE_A {
        match self.bits {
            false => SFMSDE_A::_0,
            true => SFMSDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFMSDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFMSDE_A::_1
    }
}
#[doc = "Field `SFMSDE` writer - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected."]
pub type SFMSDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSPC_SPEC, SFMSDE_A, O>;
impl<'a, const O: u8> SFMSDE_W<'a, O> {
    #[doc = "Does not allocate minimum switch time"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFMSDE_A::_0)
    }
    #[doc = "Allocate the minimum switch time equivalent to 1*QSPXLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFMSDE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately."]
    #[inline(always)]
    pub fn sfmspi(&self) -> SFMSPI_R {
        SFMSPI_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected."]
    #[inline(always)]
    pub fn sfmsde(&self) -> SFMSDE_R {
        SFMSDE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selection of SPI protocolNOTE: Serial ROM's SPI protocol is required to be set by software separately."]
    #[inline(always)]
    #[must_use]
    pub fn sfmspi(&mut self) -> SFMSPI_W<0> {
        SFMSPI_W::new(self)
    }
    #[doc = "Bit 4 - Selection of the minimum time of input output switch, when Dual SPI protocol or Quad SPI protocol is selected."]
    #[inline(always)]
    #[must_use]
    pub fn sfmsde(&mut self) -> SFMSDE_W<4> {
        SFMSDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Protocol Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmspc](index.html) module"]
pub struct SFMSPC_SPEC;
impl crate::RegisterSpec for SFMSPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmspc::R](R) reader structure"]
impl crate::Readable for SFMSPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmspc::W](W) writer structure"]
impl crate::Writable for SFMSPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSPC to value 0x10"]
impl crate::Resettable for SFMSPC_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
