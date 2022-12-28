#[doc = "Register `SFMSAC` reader"]
pub struct R(crate::R<SFMSAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFMSAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFMSAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFMSAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFMSAC` writer"]
pub struct W(crate::W<SFMSAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFMSAC_SPEC>;
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
impl From<crate::W<SFMSAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFMSAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFMAS` reader - Number of address bytes select for the serial interface"]
pub type SFMAS_R = crate::FieldReader<u8, SFMAS_A>;
#[doc = "Number of address bytes select for the serial interface\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SFMAS_A {
    #[doc = "0: 1 byte"]
    _00 = 0,
    #[doc = "1: 2 bytes"]
    _01 = 1,
    #[doc = "2: 3 bytes"]
    _10 = 2,
    #[doc = "3: 4 bytes"]
    _11 = 3,
}
impl From<SFMAS_A> for u8 {
    #[inline(always)]
    fn from(variant: SFMAS_A) -> Self {
        variant as _
    }
}
impl SFMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFMAS_A {
        match self.bits {
            0 => SFMAS_A::_00,
            1 => SFMAS_A::_01,
            2 => SFMAS_A::_10,
            3 => SFMAS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SFMAS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SFMAS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SFMAS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SFMAS_A::_11
    }
}
#[doc = "Field `SFMAS` writer - Number of address bytes select for the serial interface"]
pub type SFMAS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SFMSAC_SPEC, u8, SFMAS_A, 2, O>;
impl<'a, const O: u8> SFMAS_W<'a, O> {
    #[doc = "1 byte"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SFMAS_A::_00)
    }
    #[doc = "2 bytes"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SFMAS_A::_01)
    }
    #[doc = "3 bytes"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SFMAS_A::_10)
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SFMAS_A::_11)
    }
}
#[doc = "Field `SFM4BC` reader - Selection of instruction code automatically generated when the serial interface address width is 4 bytes"]
pub type SFM4BC_R = crate::BitReader<SFM4BC_A>;
#[doc = "Selection of instruction code automatically generated when the serial interface address width is 4 bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFM4BC_A {
    #[doc = "0: Do not use 4-byte address read instruction code"]
    _0 = 0,
    #[doc = "1: Use 4-byte address read instruction code"]
    _1 = 1,
}
impl From<SFM4BC_A> for bool {
    #[inline(always)]
    fn from(variant: SFM4BC_A) -> Self {
        variant as u8 != 0
    }
}
impl SFM4BC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFM4BC_A {
        match self.bits {
            false => SFM4BC_A::_0,
            true => SFM4BC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SFM4BC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SFM4BC_A::_1
    }
}
#[doc = "Field `SFM4BC` writer - Selection of instruction code automatically generated when the serial interface address width is 4 bytes"]
pub type SFM4BC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFMSAC_SPEC, SFM4BC_A, O>;
impl<'a, const O: u8> SFM4BC_W<'a, O> {
    #[doc = "Do not use 4-byte address read instruction code"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFM4BC_A::_0)
    }
    #[doc = "Use 4-byte address read instruction code"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFM4BC_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of address bytes select for the serial interface"]
    #[inline(always)]
    pub fn sfmas(&self) -> SFMAS_R {
        SFMAS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Selection of instruction code automatically generated when the serial interface address width is 4 bytes"]
    #[inline(always)]
    pub fn sfm4bc(&self) -> SFM4BC_R {
        SFM4BC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of address bytes select for the serial interface"]
    #[inline(always)]
    #[must_use]
    pub fn sfmas(&mut self) -> SFMAS_W<0> {
        SFMAS_W::new(self)
    }
    #[doc = "Bit 4 - Selection of instruction code automatically generated when the serial interface address width is 4 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn sfm4bc(&mut self) -> SFM4BC_W<4> {
        SFM4BC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfmsac](index.html) module"]
pub struct SFMSAC_SPEC;
impl crate::RegisterSpec for SFMSAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfmsac::R](R) reader structure"]
impl crate::Readable for SFMSAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfmsac::W](W) writer structure"]
impl crate::Writable for SFMSAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFMSAC to value 0x02"]
impl crate::Resettable for SFMSAC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
