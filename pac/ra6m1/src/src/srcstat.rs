#[doc = "Register `SRCSTAT` reader"]
pub struct R(crate::R<SRCSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCSTAT` writer"]
pub struct W(crate::W<SRCSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCSTAT_SPEC>;
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
impl From<crate::W<SRCSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OINT` reader - Output Data FIFO Full Interrupt Request Flag\n\nThe field is **modified** in some way after a read operation."]
pub type OINT_R = crate::BitReader<OINT_A>;
#[doc = "Output Data FIFO Full Interrupt Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OINT_A {
    #[doc = "0: Number of data units in the output FIFO has not become equal to or greater than the specified triggering number."]
    _0 = 0,
    #[doc = "1: Number of data units in the output FIFO has become equal to or greater than the specified triggering number."]
    _1 = 1,
}
impl From<OINT_A> for bool {
    #[inline(always)]
    fn from(variant: OINT_A) -> Self {
        variant as u8 != 0
    }
}
impl OINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OINT_A {
        match self.bits {
            false => OINT_A::_0,
            true => OINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OINT_A::_1
    }
}
#[doc = "Field `OINT` writer - Output Data FIFO Full Interrupt Request Flag"]
pub type OINT_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, SRCSTAT_SPEC, OINT_A, O>;
impl<'a, const O: u8> OINT_W<'a, O> {
    #[doc = "Number of data units in the output FIFO has not become equal to or greater than the specified triggering number."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OINT_A::_0)
    }
    #[doc = "Number of data units in the output FIFO has become equal to or greater than the specified triggering number."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OINT_A::_1)
    }
}
#[doc = "Field `IINT` reader - Input Data FIFO Empty Interrupt Request Flag\n\nThe field is **modified** in some way after a read operation."]
pub type IINT_R = crate::BitReader<IINT_A>;
#[doc = "Input Data FIFO Empty Interrupt Request Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IINT_A {
    #[doc = "0: Number of data units in the input FIFO has not become equal to or smaller than the specified triggering number."]
    _0 = 0,
    #[doc = "1: Number of data units in the input FIFO has become equal to or smaller than the specified triggering number."]
    _1 = 1,
}
impl From<IINT_A> for bool {
    #[inline(always)]
    fn from(variant: IINT_A) -> Self {
        variant as u8 != 0
    }
}
impl IINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IINT_A {
        match self.bits {
            false => IINT_A::_0,
            true => IINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IINT_A::_1
    }
}
#[doc = "Field `IINT` writer - Input Data FIFO Empty Interrupt Request Flag"]
pub type IINT_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, SRCSTAT_SPEC, IINT_A, O>;
impl<'a, const O: u8> IINT_W<'a, O> {
    #[doc = "Number of data units in the input FIFO has not become equal to or smaller than the specified triggering number."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IINT_A::_0)
    }
    #[doc = "Number of data units in the input FIFO has become equal to or smaller than the specified triggering number."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IINT_A::_1)
    }
}
#[doc = "Field `OVF` reader - Output Data FIFO Overwrite Interrupt Request Flag\n\nThe field is **modified** in some way after a read operation."]
pub type OVF_R = crate::BitReader<OVF_A>;
#[doc = "Output Data FIFO Overwrite Interrupt Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVF_A {
    #[doc = "0: Next data conversion processing is not completed."]
    _0 = 0,
    #[doc = "1: Next data conversion processing is completed."]
    _1 = 1,
}
impl From<OVF_A> for bool {
    #[inline(always)]
    fn from(variant: OVF_A) -> Self {
        variant as u8 != 0
    }
}
impl OVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVF_A {
        match self.bits {
            false => OVF_A::_0,
            true => OVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVF_A::_1
    }
}
#[doc = "Field `OVF` writer - Output Data FIFO Overwrite Interrupt Request Flag"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, SRCSTAT_SPEC, OVF_A, O>;
impl<'a, const O: u8> OVF_W<'a, O> {
    #[doc = "Next data conversion processing is not completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVF_A::_0)
    }
    #[doc = "Next data conversion processing is completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVF_A::_1)
    }
}
#[doc = "Field `UDF` reader - Output FIFO Underflow Interrupt Request Flag\n\nThe field is **modified** in some way after a read operation."]
pub type UDF_R = crate::BitReader<UDF_A>;
#[doc = "Output FIFO Underflow Interrupt Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDF_A {
    #[doc = "0: Output data FIFO has not been read out."]
    _0 = 0,
    #[doc = "1: Output data FIFO has been read out."]
    _1 = 1,
}
impl From<UDF_A> for bool {
    #[inline(always)]
    fn from(variant: UDF_A) -> Self {
        variant as u8 != 0
    }
}
impl UDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDF_A {
        match self.bits {
            false => UDF_A::_0,
            true => UDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UDF_A::_1
    }
}
#[doc = "Field `UDF` writer - Output FIFO Underflow Interrupt Request Flag"]
pub type UDF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, SRCSTAT_SPEC, UDF_A, O>;
impl<'a, const O: u8> UDF_W<'a, O> {
    #[doc = "Output data FIFO has not been read out."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UDF_A::_0)
    }
    #[doc = "Output data FIFO has been read out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UDF_A::_1)
    }
}
#[doc = "Field `FLF` reader - Flush Processing Status Flag"]
pub type FLF_R = crate::BitReader<FLF_A>;
#[doc = "Flush Processing Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLF_A {
    #[doc = "0: Flash processing is completed."]
    _0 = 0,
    #[doc = "1: Flash processing is in progress."]
    _1 = 1,
}
impl From<FLF_A> for bool {
    #[inline(always)]
    fn from(variant: FLF_A) -> Self {
        variant as u8 != 0
    }
}
impl FLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLF_A {
        match self.bits {
            false => FLF_A::_0,
            true => FLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLF_A::_1
    }
}
#[doc = "Field `CEF` reader - Conversion End Flag\n\nThe field is **modified** in some way after a read operation."]
pub type CEF_R = crate::BitReader<CEF_A>;
#[doc = "Conversion End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEF_A {
    #[doc = "0: All of the output data has not been read out."]
    _0 = 0,
    #[doc = "1: All of the output data has been read out."]
    _1 = 1,
}
impl From<CEF_A> for bool {
    #[inline(always)]
    fn from(variant: CEF_A) -> Self {
        variant as u8 != 0
    }
}
impl CEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEF_A {
        match self.bits {
            false => CEF_A::_0,
            true => CEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEF_A::_1
    }
}
#[doc = "Field `CEF` writer - Conversion End Flag"]
pub type CEF_W<'a, const O: u8> = crate::BitWriter0C<'a, u16, SRCSTAT_SPEC, CEF_A, O>;
impl<'a, const O: u8> CEF_W<'a, O> {
    #[doc = "All of the output data has not been read out."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEF_A::_0)
    }
    #[doc = "All of the output data has been read out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEF_A::_1)
    }
}
#[doc = "Field `IFDN` reader - Input FIFO Data Count"]
pub type IFDN_R = crate::FieldReader<u8, IFDN_A>;
#[doc = "Input FIFO Data Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IFDN_A(u8);
impl From<IFDN_A> for u8 {
    #[inline(always)]
    fn from(val: IFDN_A) -> Self {
        val.0 as _
    }
}
#[doc = "Field `OFDN` reader - Output FIFO Data Count"]
pub type OFDN_R = crate::FieldReader<u8, OFDN_A>;
#[doc = "Output FIFO Data Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OFDN_A(u8);
impl From<OFDN_A> for u8 {
    #[inline(always)]
    fn from(val: OFDN_A) -> Self {
        val.0 as _
    }
}
impl R {
    #[doc = "Bit 0 - Output Data FIFO Full Interrupt Request Flag"]
    #[inline(always)]
    pub fn oint(&self) -> OINT_R {
        OINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Data FIFO Empty Interrupt Request Flag"]
    #[inline(always)]
    pub fn iint(&self) -> IINT_R {
        IINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Data FIFO Overwrite Interrupt Request Flag"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Underflow Interrupt Request Flag"]
    #[inline(always)]
    pub fn udf(&self) -> UDF_R {
        UDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush Processing Status Flag"]
    #[inline(always)]
    pub fn flf(&self) -> FLF_R {
        FLF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Conversion End Flag"]
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:10 - Input FIFO Data Count"]
    #[inline(always)]
    pub fn ifdn(&self) -> IFDN_R {
        IFDN_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:15 - Output FIFO Data Count"]
    #[inline(always)]
    pub fn ofdn(&self) -> OFDN_R {
        OFDN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Output Data FIFO Full Interrupt Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn oint(&mut self) -> OINT_W<0> {
        OINT_W::new(self)
    }
    #[doc = "Bit 1 - Input Data FIFO Empty Interrupt Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn iint(&mut self) -> IINT_W<1> {
        IINT_W::new(self)
    }
    #[doc = "Bit 2 - Output Data FIFO Overwrite Interrupt Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<2> {
        OVF_W::new(self)
    }
    #[doc = "Bit 3 - Output FIFO Underflow Interrupt Request Flag"]
    #[inline(always)]
    #[must_use]
    pub fn udf(&mut self) -> UDF_W<3> {
        UDF_W::new(self)
    }
    #[doc = "Bit 5 - Conversion End Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cef(&mut self) -> CEF_W<5> {
        CEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcstat](index.html) module"]
pub struct SRCSTAT_SPEC;
impl crate::RegisterSpec for SRCSTAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [srcstat::R](R) reader structure"]
impl crate::Readable for SRCSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcstat::W](W) writer structure"]
impl crate::Writable for SRCSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x2f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCSTAT to value 0x02"]
impl crate::Resettable for SRCSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
